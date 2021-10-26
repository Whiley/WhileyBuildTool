// Copyright 2011 The Whiley Project Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
package wy.cfg;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import jbfs.core.Build;
import jbfs.core.Content;
import jbfs.util.Pair;
import jbfs.util.Trie;

public class ConfigFile implements Build.Artifact {
	// =========================================================================
	// Content Type
	// =========================================================================

	public static final Content.Type<ConfigFile> ContentType = new Content.Type<>() {
		@Override
		public ConfigFile read(Trie id, InputStream input, Content.Registry registry) throws IOException {
			ConfigFileLexer lexer = new ConfigFileLexer(input);
			ConfigFileParser parser = new ConfigFileParser(id, lexer.scan());
			return parser.read();
		}

		@Override
		public void write(OutputStream output, ConfigFile value) {
			// for now
			throw new UnsupportedOperationException();
		}

		@Override
		public String toString() {
			return "Content-Type: toml";
		}

		@Override
		public String getSuffix() {
			return "toml";
		}
	};

	// =========================================================================
	// Node kinds
	// =========================================================================

	public static final int DECL_mask = 0b00010000;
	public static final int DECL_section = DECL_mask + 0;
	public static final int DECL_keyvalue = DECL_mask + 1;

	// =========================================================================
	// Constructors
	// =========================================================================
	private final Trie path;
	/**
	 * The list of declarations which make up this configuration.
	 */
	private ArrayList<Declaration> declarations;

	public ConfigFile(Trie path) {
		this.declarations = new ArrayList<>();
		this.path = path;
	}

	public ConfigFile(Trie path, Collection<Declaration> declarations) {
		this.declarations = new ArrayList<>(declarations);
		this.path = path;
	}

	@Override
	public Trie getPath() {
		return path;
	}

	@Override
	public Content.Type<ConfigFile> getContentType() {
		return ConfigFile.ContentType;
	}

	@Override
	public List<? extends Build.Artifact> getSourceArtifacts() {
		return Collections.EMPTY_LIST;
	}

	public static interface Declaration {

	}

	public List<Declaration> getDeclarations() {
		return declarations;
	}

	public void setDeclarations(List<Declaration> declarations) {
		this.declarations = new ArrayList<>(declarations);
	}

	/**
	 * Construct a configuration wrapper for this file. This ensures that the
	 * contents of the file meets a give configuration schema.
	 *
	 * @param schema The schema to use for the resulting configuration
	 * @param strict indicates whether or not to allow spurious entries in the
	 *               configuration file.
	 * @return
	 */
	public Configuration toConfiguration(Configuration.Schema schema, boolean strict) {
		return new Wrapper(schema, strict);
	}

	public static class Table implements Declaration {
		private final String name;
		private final ArrayList<KeyValuePair> contents;
		public Table(String name, List<KeyValuePair> contents) {
			this.name = name;
			this.contents = new ArrayList<>(contents);
		}

		public String getName() {
			return name;
		}

		public List<KeyValuePair> getContents() {
			return contents;
		}
	}

	/**
	 * Maps a given key to a given value.
	 *
	 * @author David J. Pearce
	 *
	 */
	public static class KeyValuePair extends Pair<String,Object> implements Declaration {

		public KeyValuePair(String key, Object value) {
			super(key, value);
		}

		public String getKey() {
			return first();
		}

		public Object getValue() {
			return second();
		}
	}

	private KeyValuePair getKeyValuePair(Trie key, List<? extends Declaration> decls) {
		String table = key.parent().toString();
		//
		for (int i = 0; i != decls.size(); ++i) {
			Declaration decl = decls.get(i);
			if (key.size() > 1 && decl instanceof Table) {
				Table s = (Table) decl;
				if (s.getName().equals(table)) {
					return getKeyValuePair(key.subpath(key.size() - 1, key.size()), s.getContents());
				}
			} else if (decl instanceof KeyValuePair && key.size() == 1) {
				KeyValuePair p = (KeyValuePair) decl;
				if (p.getKey().toString().equals(key.get(0))) {
					return p;
				}
			}
		}
		return null;
	}

	private class Wrapper implements Configuration {
		/**
		 * The schema to which this configuration file adheres.
		 */
		private final Configuration.Schema schema;

		/**
		 * Indicate whether or not to allow spurios entries (which are then hidden)
		 */
		private final boolean strict;

		public Wrapper(Configuration.Schema schema, boolean strict) {
			this.schema = schema;
			this.strict = strict;
			validate();
		}

		@Override
		public Schema getConfigurationSchema() {
			return schema;
		}

		@Override
		public boolean hasKey(Trie key) {
			// Find the key-value pair
			KeyValuePair kvp = getKeyValuePair(key, declarations);
			// If didn't find a value, still might have default
			if(kvp == null && schema.isKey(key)) {
				// Get the descriptor for this key
				Configuration.KeyValueDescriptor<?> descriptor = schema.getDescriptor(key);
				// Check whether have a default
				return descriptor.hasDefault();
			} else {
				return kvp != null;
			}
		}

		@Override
		public <T> T get(Class<T> kind, Trie key) {
			// Get the descriptor for this key
			Configuration.KeyValueDescriptor<?> descriptor = schema.getDescriptor(key);
			// Find the key-value pair
			KeyValuePair kvp = getKeyValuePair(key, declarations);
			if (kvp == null && descriptor.hasDefault()) {
				return (T) descriptor.getDefault();
			} else if (kvp != null) {
				// Extract the value
				Object value = kvp.getValue();
				// Sanity check the expected kind
				if (!kind.isInstance(value)) {
					throw new IllegalArgumentException("incompatible key access: expected " + kind.getSimpleName()
							+ " got " + descriptor.getType().getSimpleName());
				}
				//
				if (descriptor != null) {
					// Convert into value
					return (T) value;
				} else {
					throw new IllegalArgumentException("hidden key access: " + key);
				}
			} else {
				throw new IllegalArgumentException("invalid key access: " + key);
			}
		}

		@Override
		public <T> void write(Trie key, T value) {
			throw new UnsupportedOperationException();
		}

		@Override
		public List<Trie> matchAll(Trie filter) {
			ArrayList<Trie> matches = new ArrayList<>();
			match(Trie.ROOT,filter,declarations,matches);
			return matches;
		}

		@Override
		public String toString() {
			List<Trie> keys = matchAll(Trie.fromString("**/*"));
			String r = "{";
			for(int i=0;i!=keys.size();++i) {
				Trie ith = keys.get(i);
				r = (i == 0) ? r : r + ",";
				r += ith + "=" + get(ith);
			}
			return r + "}";
		}

		private Object get(Trie key) {
			// Get the descriptor for this key
			Configuration.KeyValueDescriptor<?> descriptor = schema.getDescriptor(key);
			// Find the key-value pair
			KeyValuePair kvp = getKeyValuePair(key, declarations);
			if (kvp == null && descriptor.hasDefault()) {
				return descriptor.getDefault();
			} else if (kvp != null) {
				// Extract the value
				return kvp.getValue();
			} else {
				throw new IllegalArgumentException("invalid key access: " + key);
			}
		}

		private void match(Trie id, Trie filter, List<? extends Declaration> declarations, ArrayList<Trie> matches) {
			for (int i = 0; i != declarations.size(); ++i) {
				Declaration decl = declarations.get(i);
				if (decl instanceof Table) {
					Table table = (Table) decl;
					// FIXME: could be more efficient!
					Trie tid = id.append(Trie.fromString(table.getName()));
					match(tid, filter, table.getContents(), matches);
				} else if (decl instanceof KeyValuePair) {
					KeyValuePair kvp = (KeyValuePair) decl;
					Trie match = id.append(kvp.getKey().toString());
					if (filter.matches(match)) {
						matches.add(match);
					}
				}
			}
		}

		private void validate() {
			List<KeyValueDescriptor<?>> descriptors = schema.getDescriptors();
			// Matched holds all concrete key-value pairs which are matched. This allows us
			// to identify any which were not matched and, hence, are invalid
			Set<Trie> matched = new HashSet<>();
			// Validate all descriptors against given values.
			for (int i = 0; i != descriptors.size(); ++i) {
				KeyValueDescriptor descriptor = descriptors.get(i);
				// Sanity check the expected kind
				Class<?> kind = descriptor.getType();
				// Identify all matching keys
				List<Trie> results = matchAll(descriptor.getFilter());
				// Sanity check whether required
				if(results.size() == 0 && descriptor.isRequired()) {
					throw new IllegalArgumentException("missing key value: " + descriptor.getFilter());
				}
				// Check all matching keys
				for (Trie id : results) {
					// Find corresponding key value pair.
					KeyValuePair kvp = getKeyValuePair(id, declarations);
					// NOTE: kvp != null
					if (!kind.isInstance(kvp.getValue())) {
						throw new IllegalArgumentException("invalid key value (expected " + kind.getSimpleName() + ")");
					} else if (!descriptor.isValid(kvp.getValue())) {
						// Identified invalid key-value pair
						throw new IllegalArgumentException("invalid key value");
					}
				}
				// Remember every matched attribute
				matched.addAll(results);
			}
			if (strict) {
				// Check whether any unmatched key-valid pairs exist or not
				List<Trie> all = matchAll(Trie.fromString("**/*"));
				for (int i = 0; i != all.size(); ++i) {
					Trie id = all.get(i);
					if (!matched.contains(id)) {
						// Found unmatched attribute
						KeyValuePair kvp = getKeyValuePair(id, declarations);
						throw new IllegalArgumentException("invalid key: " + id);
					}
				}
			}
			// Done
		}
	}
}