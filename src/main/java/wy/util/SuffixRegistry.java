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
package wy.util;

import java.io.File;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Content.Type;
import jbuildstore.core.Key;

public class SuffixRegistry<T extends Content> implements Key.EncoderDecoder<Trie, T, String> {
	private final HashMap<String, Content.Type<? extends T>> registry = new HashMap<>();

	/**
	 * Register a new content type with this registry.
	 *
	 * @param suffix
	 * @param ct
	 */
	public void add(Content.Type<? extends T> ct) {
		this.registry.put(ct.suffix(), ct);
	}

	/**
	 * Register a list of zero or more content types with this registry.
	 * @param cts
	 */
	public void addAll(List<Content.Type<? extends T>> cts) {
		for(Content.Type<? extends T> ct : cts) {
			add(ct);
		}
	}

	@Override
	public String encode(Type<? extends T> type, Trie key) {
		for (Map.Entry<String, Content.Type<? extends T>> e : registry.entrySet()) {
			if (e.getValue() == type) {
				return key.toString().replace('/', File.separatorChar) + "." + e.getKey();
			}
		}
		return null;
	}

	@Override
	public Trie decodeKey(String t) {
		int i = t.lastIndexOf('.');
		if (i >= 0) {
			t = t.substring(0,i);
		}
		return Trie.fromString(t.replace(File.separatorChar, '/'));
	}

	@Override
	public Type decodeType(String t) {
		int i = t.lastIndexOf('.');
		if (i >= 0) {
			String suffix = t.substring(i + 1);
			return registry.get(suffix);
		} else {
			return null;
		}
	}
}
