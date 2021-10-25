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
package wybt.cfg;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.regex.Pattern;

import jbfs.util.Trie;

/**
 * A configuration provides a generic key-value store for which the backing is
 * not specifically determined. For example, it could be backed by a database or
 * simply a configuration file.
 *
 * @author David J. Pearce
 *
 */
public interface Configuration {

	/**
	 * Get the schema associated with the given configuration.
	 *
	 * @return
	 */
	public Schema getConfigurationSchema();

	/**
	 * Check whether a value exists for a given key.
	 *
	 * @param key
	 * @return
	 */
	public <T> boolean hasKey(Trie key);

	/**
	 * Get the value associated with a given key. If no such key exists, an
	 * exception is raised. Every value returned is valid with respect to the
	 * schema.
	 */
	public <T> T get(Class<T> kind, Trie key);

	/**
	 * Associate a given value with a given key in the configuration. This will
	 * create a new key if none existed before. The given value must conform to the
	 * schema for this configuration, otherwise an exception is raised.
	 *
	 * @param key
	 * @param value
	 */
	public <T> void write(Trie key, T value);

	/**
	 * Determine all matching keys in this configuration.
	 *
	 * @param filter
	 * @return
	 */
	public List<Trie> matchAll(Trie filter);

	/**
	 * Determines what values are permitted and required for this configuration.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface Schema {
		/**
		 * Check whether the give key is known to this schema or not.
		 *
		 * @param key
		 * @return
		 */
		public boolean isKey(Trie key);

		/**
		 * Get the descriptor associated with a given key.
		 *
		 * @param key
		 * @return
		 */
		public KeyValueDescriptor<?> getDescriptor(Trie key);

		/**
		 * Get the list of all descriptors in this schema.
		 *
		 * @return
		 */
		public List<KeyValueDescriptor<?>> getDescriptors();

	}

	/**
	 * Root of all errors arising from configuration problems.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface Error {

	}

	/**
	 * Represents a simple empty configuration. This is useful for handling cases
	 * where e.g. a configuration file cannot be located.
	 */
	public static Configuration EMPTY(Schema schema) {
		return new Configuration() {

			@Override
			public Schema getConfigurationSchema() {
				return schema;
			}

			@Override
			public <T> T get(Class<T> kind, Trie key) {
				Configuration.KeyValueDescriptor<?> descriptor = schema.getDescriptor(key);
				if(descriptor != null && descriptor.hasDefault()) {
					Object value = descriptor.getDefault();
					if (!kind.isInstance(value)) {
						throw new IllegalArgumentException("incompatible key access: expected " + kind.getSimpleName()
								+ " got " + descriptor.getType().getSimpleName());
					}
					return (T) value;
				}
				throw new IllegalArgumentException("invalid key access: " + key);
			}

			@Override
			public <T> void write(Trie key, T value) {
				throw new IllegalArgumentException("invalid key access: " + key);
			}

			@Override
			public List<Trie> matchAll(Trie filter) {
				// FIXME: need really to implement this method somehow!

				return Collections.EMPTY_LIST;
			}

			@Override
			public <T> boolean hasKey(Trie key) {
				Configuration.KeyValueDescriptor<?> descriptor = schema.getDescriptor(key);
				return descriptor != null && descriptor.hasDefault();
			}
		};
	}

	/**
	 * A simple schema which contains no keys.
	 */
	public static final Configuration.Schema EMPTY_SCHEMA = new Configuration.Schema() {

		@Override
		public boolean isKey(Trie key) {
			return false;
		}

		@Override
		public KeyValueDescriptor<?> getDescriptor(Trie key) {
			throw new IllegalArgumentException("invalid key: " + key);
		}

		@Override
		public List<KeyValueDescriptor<?>> getDescriptors() {
			return Collections.EMPTY_LIST;
		}
	};

	/**
	 * Construct a single schema from one or more schemas.
	 *
	 * @param schemas
	 * @return
	 */
	public static Schema toCombinedSchema(Configuration.Schema... schemas) {
		// FIXME: Sanity check schemas?
		//
		return new Schema() {

			@Override
			public boolean isKey(Trie key) {
				for (int i = 0; i != schemas.length; ++i) {
					if (schemas[i].isKey(key)) {
						return true;
					}
				}
				return false;
			}

			@Override
			public KeyValueDescriptor<?> getDescriptor(Trie key) {
				for (int i = 0; i != schemas.length; ++i) {
					Schema schema = schemas[i];
					//
					if (schema.isKey(key)) {
						return schema.getDescriptor(key);
					}
				}
				//
				throw new IllegalArgumentException("invalid key access: " + key);
			}

			@Override
			public List<KeyValueDescriptor<?>> getDescriptors() {
				ArrayList<KeyValueDescriptor<?>> descriptors = new ArrayList<>();
				for (int i = 0; i != schemas.length; ++i) {
					descriptors.addAll(schemas[i].getDescriptors());
				}
				return descriptors;
			}
		};
	}

	/**
	 * Construct a schema from a given array of KeyValueDescriptors.
	 *
	 * @param required
	 *            The set of required key-value pairs.
	 * @param optional
	 *            The set of optional key-value pairs.
	 * @return
	 */
	public static Schema fromArray(KeyValueDescriptor<?>... descriptors) {
		// Finally construct the schema
		return new Schema() {

			@Override
			public KeyValueDescriptor<?> getDescriptor(Trie key) {
				for (int i = 0; i != descriptors.length; ++i) {
					KeyValueDescriptor<?> descriptor = descriptors[i];
					if (descriptor.getFilter().matches(key)) {
						return descriptor;
					}
				}
				throw new IllegalArgumentException("invalid key \"" + key + "\"");
			}

			@Override
			public boolean isKey(Trie key) {
				for (int i = 0; i != descriptors.length; ++i) {
					KeyValueDescriptor<?> descriptor = descriptors[i];
					if (descriptor.getFilter().matches(key)) {
						return true;
					}
				}
				return false;
			}

			@Override
			public List<KeyValueDescriptor<?>> getDescriptors() {
				return Arrays.asList(descriptors);
			}

		};
	}

	/**
	 * Provides a generic mechanism for describing a key-value pair and ensuring
	 * that all values in a given configuration conform. This includes ensuring they
	 * have the right type, and that they meet given constraints.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface KeyValueDescriptor<T> {
		/**
		 * Get the key filter associated with this descriptor.
		 *
		 * @return
		 */
		public Trie getFilter();

		/**
		 * Get the description associated with this descriptor.
		 *
		 * @return
		 */
		public String getDescription();

		/**
		 * Get the type associated with this validator, which could be e.g.
		 * <code>String</code>, <code>Boolean</code> or <code>Integer</code>.
		 *
		 * @return
		 */
		public Class<T> getType();

		/**
		 * Check whether at least one matching key-value pair is required for a given
		 * schema
		 *
		 * @return
		 */
		public boolean isRequired();

		/**
		 * Determine whether or not this descriptor describes a default value for the
		 * key.
		 *
		 * @return
		 */
		public boolean hasDefault();

		/**
		 * Get the default value for this field (if applicable).
		 *
		 * @return
		 */
		public T getDefault();

		/**
		 * Check whether a given value is actual valid. For example, integer values may
		 * be prevented from being negative, etc. Likewise, string values representing
		 * version numbers may need to conform to a given regular expression, etc.
		 *
		 * @param value
		 * @return
		 */
		public boolean isValid(T value);
	}

	/**
	 * A simple base class for arbitrary validators.
	 *
	 * @author David J. Pearce
	 *
	 * @param <T>
	 */
	public static abstract class AbstractDescriptor<T> implements KeyValueDescriptor<T> {
		private final Trie key;
		private final String description;
		private final boolean required;
		private final T defaulT;
		private final Class<T> type;

		public AbstractDescriptor(Trie key, String description, Class<T> type, boolean required) {
			this.key = key;
			this.description = description;
			this.type = type;
			this.required = required;
			this.defaulT = null;
		}

		public AbstractDescriptor(Trie key, String description, Class<T> type, T defaulT) {
			this.key = key;
			this.description = description;
			this.type = type;
			this.defaulT = defaulT;
			this.required = false;
		}

		@Override
		public Trie getFilter() {
			return key;
		}

		@Override
		public String getDescription() {
			return description;
		}

		@Override
		public Class<T> getType() {
			return type;
		}

		@Override
		public boolean isRequired() {
			return required;
		}

		@Override
		public boolean hasDefault() {
			return defaulT != null;
		}

		@Override
		public T getDefault() {
			return defaulT;
		}

		@Override
		public boolean isValid(T value) {
			return true;
		}

		@Override
		public String toString() {
			return key + ":\"" + description + "\": " + type.getSimpleName() + ":" + defaulT + ":" + required;
		}
	}

	/**
	 * Represents an unbound string key-value pair. That is, any string is
	 * permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @return
	 */
	public static KeyValueDescriptor<String> UNBOUND_STRING(Trie key, String description, boolean required) {
		return new AbstractDescriptor<>(key, description, String.class, required) {

		};
	}

	/**
	 * Represents an unbound string key-value pair with a default value. That is,
	 * any string is permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param defaulT
	 *            Default to return in case this attribute is not specified.
	 * @return
	 */
	public static KeyValueDescriptor<String> UNBOUND_STRING(Trie key, String description, String defaulT) {
		return new AbstractDescriptor<>(key, description, String.class, defaulT) {

		};
	}

	/**
	 * Represents a key-value pair where the value is a string conforming to a given
	 * regex.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @param regex
	 *            The regular expression to which instances of this kvp must
	 *            conform.
	 * @return
	 */
	public static KeyValueDescriptor<String> REGEX_STRING(Trie key, String description, boolean required,
			Pattern regex) {
		return new AbstractDescriptor<>(key, description, String.class, required) {
			@Override
			public boolean isValid(String str) {
				return regex.matcher(str.toString()).matches();
			}
		};
	}

	/**
	 * Represents a key-value pair where the value is a string conforming to a given
	 * regex, and a default value is providfed.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param defaulT
	 *            Default to return in case this attribute is not specified.
	 * @param regex
	 *            The regular expression to which instances of this kvp must
	 *            conform.
	 * @return
	 */
	public static KeyValueDescriptor<String> REGEX_STRING(Trie key, String description, String defaulT,
			Pattern regex) {
		KeyValueDescriptor<String> desc = new AbstractDescriptor<>(key, description, String.class,
				defaulT) {
			@Override
			public boolean isValid(String str) {
				return regex.matcher(str.toString()).matches();
			}
		};
		// Sanity check default value
		checkDefaultValue(desc, defaulT);
		// Done
		return desc;
	}

	/**
	 * Represents an unbound integer key-valid pair. That is, any integer is
	 * permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @return
	 */
	public static KeyValueDescriptor<BigInteger> UNBOUND_INTEGER(Trie key, String description, boolean required) {
		return new AbstractDescriptor<>(key, description, BigInteger.class, required) {

		};
	}

	/**
	 * Represents an unbound integer key-valid pair with a default value. That is,
	 * any integer is permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param defaulT
	 *            Default to return in case this attribute is not specified.
	 * @return
	 */
	public static KeyValueDescriptor<BigInteger> UNBOUND_INTEGER(Trie key, String description,
			BigInteger defaulT) {
		return new AbstractDescriptor<>(key, description, BigInteger.class, defaulT) {

		};
	}

	/**
	 * Returns an integer key-value descriptor which ensures the given value is
	 * greater or equal to a given lower bound.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @param low
	 *            No valid value is below this bound.
	 * @return
	 */
	public static KeyValueDescriptor<BigInteger> BOUND_INTEGER(Trie key, String description, boolean required,
			final int low) {
		return new AbstractDescriptor<>(key, description, BigInteger.class, required) {
			@Override
			public boolean isValid(BigInteger value) {
				int v = value.intValue();
				return v >= low;
			}
		};
	}

	/**
	 * Returns an integer key-value descriptor which ensures the given value is
	 * greater or equal to a given lower bound.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param defaulT
	 *            Default to return in case this attribute is not specified.
	 * @param low
	 *            No valid value is below this bound.
	 * @return
	 */
	public static KeyValueDescriptor<BigInteger> BOUND_INTEGER(Trie key, String description, BigInteger defaulT,
			final int low) {
		KeyValueDescriptor<BigInteger> desc = new AbstractDescriptor<>(key, description, BigInteger.class,
				defaulT) {
			@Override
			public boolean isValid(BigInteger value) {
				int v = value.intValue();
				return v >= low;
			}
		};
		// Sanity check default value
		checkDefaultValue(desc, defaulT);
		// Done
		return desc;
	}

	/**
	 * Returns an integer key-value descriptor which ensures the given value is
	 * greater-or-equal to a given lower bound and less-or-equal to a given upper
	 * bound.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @param low
	 *            No valid value is below this bound.
	 * @param high
	 *            No valid value is above this bound.
	 * @return
	 */
	public static KeyValueDescriptor<BigInteger> BOUND_INTEGER(Trie key, String description, BigInteger defaulT,
			final int low, final int high) {
		return new AbstractDescriptor<>(key, description, BigInteger.class, defaulT) {
			@Override
			public boolean isValid(BigInteger value) {
				int v = value.intValue();
				return v >= low && v <= high;
			}
		};
	}

	/**
	 * Returns a decimal key-value descriptor which ensures the given value is
	 * bound.
	 *
	 * @param key         Identifies keys associated with this descriptor.
	 * @param description Description to use for this descriptor.
	 * @param required    Indicates whether at least one match is required for this
	 *                    descriptor for a given schema
	 * @param low         No valid value is below this bound.
	 * @param high        No valid value is above this bound.
	 * @return
	 */
	public static KeyValueDescriptor<BigDecimal> BOUND_DECIMAL(Trie key, String description, BigDecimal defaulT,
			final double low, final double high) {
		return new AbstractDescriptor<>(key, description, BigDecimal.class, defaulT) {
			@Override
			public boolean isValid(BigDecimal value) {
				int v = value.intValue();
				return v >= low && v <= high;
			}
		};
	}

	/**
	 * Represents an unbound boolean key-valid pair. That is, any boolean is
	 * permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param required
	 *            Indicates whether at least one match is required for this
	 *            descriptor for a given schema
	 * @return
	 */
	public static KeyValueDescriptor<Boolean> UNBOUND_BOOLEAN(Trie key, String description,
			boolean required) {
		return new AbstractDescriptor<>(key, description, Boolean.class, required) {

		};
	}

	/**
	 * Represents an unbound boolean key-valid pair. That is, any boolean is
	 * permitted.
	 *
	 * @param key
	 *            Identifies keys associated with this descriptor.
	 * @param description
	 *            Description to use for this descriptor.
	 * @param defaulT
	 *            Default to return in case this attribute is not specified.
	 * @return
	 */
	public static KeyValueDescriptor<Boolean> UNBOUND_BOOLEAN(Trie key, String description,
			Boolean defaulT) {
		return new AbstractDescriptor<>(key, description, Boolean.class, defaulT) {

		};
	}

	/**
	 * Represents an unbound string array. That is any number of elements are
	 * permitted.
	 *
	 * @param key
	 * @param description
	 * @param required
	 * @return
	 */
	public static KeyValueDescriptor<Object[]> UNBOUND_STRING_ARRAY(Trie key, String description,
			boolean required) {
		return new AbstractDescriptor<>(key, description, Object[].class, required) {
			@Override
			public boolean isValid(Object[] value) {
				for (int i = 0; i != value.length; ++i) {
					if (!(value[i] instanceof String)) {
						return false;
					}
				}
				return true;
			}
		};
	}

	/**
	 * Represents an unbound string array with a default value. That is any number
	 * of elements are permitted.
	 *
	 * @param key
	 * @param description
	 * @param required
	 * @return
	 */
	public static KeyValueDescriptor<Object[]> UNBOUND_STRING_ARRAY(Trie key, String description,
			Object[] defaulT) {
		AbstractDescriptor<Object[]> desc = new AbstractDescriptor<>(key, description, Object[].class,
				defaulT) {
			@Override
			public boolean isValid(Object[] value) {
				for (int i = 0; i != value.length; ++i) {
					if (!(value[i] instanceof String)) {
						return false;
					}
				}
				return true;
			}
		};
		// Sanity check default value
		checkDefaultValue(desc, defaulT);
		// Done
		return desc;
	}


	/**
	 * Represents an unbound array with a default value. That is any number
	 * of elements are permitted.
	 *
	 * @param key
	 * @param description
	 * @param required
	 * @return
	 */
	public static <T> KeyValueDescriptor<Object[]> UNBOUND_ARRAY(Trie key, String description, Class<T> kind,
			Object[] defaulT) {
		AbstractDescriptor<Object[]> desc = new AbstractDescriptor<>(key, description, Object[].class, defaulT) {
			@Override
			public boolean isValid(Object[] value) {
				for (int i = 0; i != value.length; ++i) {
					if (!(kind.isInstance(value[i]))) {
						return false;
					}
				}
				return true;
			}
		};
		// Sanity check default value
		checkDefaultValue(desc, defaulT);
		// Done
		return desc;
	}

	public static <T> void checkDefaultValue(KeyValueDescriptor<T> desc, T defaulT) {
		if (!desc.isValid(defaulT)) {
			throw new IllegalArgumentException("Invalid default value");
		}
	}
}
