package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;


public class BasicFile implements Content {

	public static Content.Type<BasicFile> ContentType = new Content.Type<>() {
		@Override
		public BasicFile read(InputStream input) throws IOException {
			throw new UnsupportedOperationException();
		}

		@Override
		public void write(OutputStream output, BasicFile value) throws IOException {
			throw new UnsupportedOperationException();
		}

		@Override
		public String suffix() {
			return "bil";
		}
	};

	private final HashMap<Integer,Stmt> stmts;

	public BasicFile(HashMap<Integer,Stmt> stmts) {
		this.stmts = new HashMap<>();
	}

	@Override
	public Type<BasicFile> contentType() {
		return ContentType;
	}

	// =========================================================
	// Statements
	// =========================================================

	public interface Stmt {
		/**
		 * Responsible for printing something to the console.
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Print implements Stmt {
			private final Expr expr;

			public Print(Expr expr) {
				this.expr = expr;
			}

			public Expr getExpr() {
				return expr;
			}
		}

		/**
		 * Unconditional branching statement.
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Goto implements Stmt {
			/**
			 * Identifies the line number to which control should be transfered.
			 */
			private final int target;

			public Goto(int target) {
				this.target = target;
			}

			public int getTarget() {
				return target;
			}
		}
	}

	public interface Expr {
		/**
		 * Represents different kinds of constants found in a given expression (e.g.
		 * string literals, integer literals, etc).
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Constant implements Expr {
			private final Object constant;

			public Constant(Object constant) {
				this.constant = constant;
			}

			public <T> T getAs(Class<T> kind) {
				if (kind.isInstance(constant)) {
					return (T) constant;
				} else {
					throw new IllegalArgumentException("invalid constant encountered");
				}
			}
		}
	}
}
