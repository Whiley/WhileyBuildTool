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
package wybt.lang;

import java.util.List;

import jbfs.util.Trie;

public class Syntactic {
	public static interface Heap {

	}

	public static interface SyntacticItem extends Comparable<SyntacticItem> {

		public int getOpcode();

		/**
		 * Get the number of operands in this bytecode
		 *
		 * @return
		 */
		public int size();

		/**
		 * Return the ith top-level child in this bytecode.
		 *
		 * @param i
		 * @return
		 */
		public SyntacticItem get(int i);

		/**
		 * Return the top-level children in this bytecode.
		 *
		 * @return
		 */
		public SyntacticItem[] getAll();

		/**
		 * Get the index of this item in the parent's heap.
		 *
		 * @return
		 */
		public int getIndex();

		/**
		 * Get any data associated with this item. This will be null if no data is
		 * associated.
		 *
		 * @return
		 */
		public byte[] getData();

		/**
		 * Get the first syntactic item of a given kind which refers to this item.
		 *
		 * @param kind
		 * @return
		 */
		public <T extends SyntacticItem> T getParent(Class<T> kind);

		/**
		 * Get all syntactic items of a given kind which refer to this item.
		 *
		 * @param kind
		 * @return
		 */
		public <T extends SyntacticItem> List<T> getParents(Class<T> kind);

		/**
		 * Get the first syntactic item of a given kind which refers directly or
		 * indirectly to this item.
		 *
		 * @param kind
		 * @return
		 */
		public <T extends SyntacticItem> T getAncestor(Class<T> kind);

		/**
		 * Create a new copy of the given syntactic item with the given operands.
		 * The number of operands must match <code>size()</code> for this item, and
		 * be of appropriate type.
		 *
		 * @param operands
		 * @return
		 */
		public SyntacticItem clone(SyntacticItem[] operands);
	}

	/**
	 * A marker represents some kind of information which should be communicated to
	 * the user. For example, a syntax error or a warning. However, there are other
	 * possible markers which could be used such as for reporting possible
	 * refactoring positions, etc.
	 *
	 * @author David J. Pearce
	 *
	 */
	public static interface Marker {
		/**
		 * Get the message associated with this marker.
		 *
		 * @return
		 */
		public String getMessage();

		/**
		 * Get the syntactic item to which this marker is associated.
		 *
		 * @return
		 */
		public SyntacticItem getTarget();

		/**
		 * Get concrete path of enclosing source file.
		 *
		 * @return
		 */
		public Trie getSource();
	}
}
