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
package wy.lang;

import java.io.PrintStream;
import java.util.List;
import java.util.function.Predicate;

import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;

public class Syntactic {
	/**
	 * Represents an exception which has been raised on a synctic item. The purpose
	 * of the exception is to identify this item in order that better information
	 * can be reported.
	 *
	 * @author David J. Pearce
	 *
	 */
	public class Exception extends RuntimeException {
		public void outputSourceError(PrintStream out, boolean brief) {
			throw new IllegalArgumentException();
		}
	}

	/**
	 * A syntactic heap represents a collection of syntactic items.
	 *
	 * @author David J. Pearce
	 *
	 */
	public static interface Heap extends Build.Artifact {

		/**
		 * Get the number of items in the heap.
		 *
		 * @return
		 */
		public int size();

		/**
		 * Return the ith syntactic item in this heap. This may return null if the
		 * item in question has been garbage collected.
		 *
		 * @param index
		 * @return
		 */
		public SyntacticItem getSyntacticItem(int ith);

		/**
		 * Find first heap item matching the given constraint.
		 *
		 * @param <T>
		 * @param kind
		 * @param where
		 * @return
		 */
		public <T extends SyntacticItem> T match(Class<T> kind, Predicate<SyntacticItem> where);

		/**
		 * Match all heap items matching the given constraint.
		 *
		 * @param <T>
		 * @param kind
		 * @param where
		 * @return
		 */
		public <T extends SyntacticItem> List<T> matchAll(Class<T> kind, Predicate<SyntacticItem> where);

		/**
		 * Get an associated attribute map with this syntactic heap.
		 *
		 * @param <T>
		 * @param <S>
		 * @param kind
		 * @return
		 */
		public <T, S extends AttributeMap<T>> S getAttributeMap(Class<T> kind);
	}

	public static interface SyntacticItem extends Comparable<SyntacticItem> {

		/**
		 * Get the opcode associated with this item.
		 *
		 * @return
		 */
		public int getOpcode();

		/**
		 * Get the number of operands in this bytecode
		 *
		 * @return
		 */
		public int size();

		/**
		 * Return the ith top-level operand in this bytecode.
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
		 * Get the enclosing heap for this item.
		 */
		public Heap getHeap();

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

	/**
	 * A span associates a given syntactic item with a contiguous region of text in
	 * the original source file.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface Span {
		public int getStart();

		public int getEnd();
	}

	/**
	 *
	 * @author djp
	 *
	 * @param <T>
	 */
	public interface AttributeMap<T> {
		public T get(SyntacticItem item);
	}
}
