package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import jbuildstore.core.Content;
import jsynheap.io.HeapReader;
import jsynheap.io.HeapWriter;
import jsynheap.lang.Syntactic;
import jsynheap.lang.Syntactic.Item;
import jsynheap.util.AbstractCompilationUnit;
import jsynheap.util.AbstractItem;
import jsynheap.util.SectionedSchema;
import jsynheap.util.SectionedSchema.Section;

public class BasicFile extends AbstractCompilationUnit implements Content {

	public static Content.Type<BasicFile> ContentType = new Content.Type<>() {
		@Override
		public BasicFile read(InputStream input) throws IOException {
			return (BasicFile) new Reader(input).read();
		}

		@Override
		public void write(OutputStream output, BasicFile value) throws IOException {
			new Writer(output).write(value);
		}

		@Override
		public String suffix() {
			return "bil";
		}
	};

	public BasicFile() {
		super();
	}

	private BasicFile(int root, Syntactic.Item[] items) {
		// Allocate every item into this heap
		for (int i = 0; i != items.length; ++i) {
			syntacticItems.add(items[i]);
			items[i].allocate(this, i);
		}
		// Set the distinguished root item
		setRootItem(getSyntacticItem(root));
	}

	@Override
	public Type<BasicFile> contentType() {
		return ContentType;
	}

	// =========================================================
	// Statements
	// =========================================================

	public static final int STMT_PRINT = 16;
	public static final int STMT_GOTO = 17;
	//
	public static final int EXPR_CONST = 32;

	public interface Stmt extends Syntactic.Item {

		/**
		 * Base for all statements
		 * @author djp
		 *
		 */
		public static abstract class Base extends AbstractItem {
			public Base(int opcode, Item... operands) {
				super(opcode, operands);
			}

			public int getLineNumber() {
				Value.Int l = (Value.Int) super.get(0);
				return l.get().intValueExact();
			}
		}
		/**
		 * Responsible for printing something to the console.
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Print extends Base implements Stmt {
			public Print(int line, Expr expr) {
				this(new Value.Int(line),expr);
			}

			public Print(Value.Int line, Expr expr) {
				super(STMT_PRINT,line,expr);
			}

			public Expr getExpr() {
				return (Expr) super.get(1);
			}

			@Override
			public Item clone(Item[] operands) {
				return new Print((Value.Int) operands[0], (Expr) operands[1]);
			}

			public static final Descriptor DESCRIPTOR = new Descriptor(Operands.TWO, Data.ZERO, "STMT_print") {
				@SuppressWarnings("unchecked")
				@Override
				public Syntactic.Item construct(int opcode, Syntactic.Item[] operands, byte[] data) {
					return new Print((Value.Int) operands[0], (Expr) operands[1]);
				}
			};
		}

		/**
		 * Unconditional branching statement.
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Goto extends Base implements Stmt {
			public Goto(int line, int target) {
				this(new Value.Int(line), new Value.Int(target));
			}

			public Goto(Value.Int line, Value.Int target) {
				super(STMT_GOTO, line, target);
			}

			public int getTarget() {
				Value.Int l = (Value.Int) super.get(1);
				return l.get().intValueExact();
			}

			@Override
			public Item clone(Item[] operands) {
				return new Goto((Value.Int) operands[0], (Value.Int) operands[1]);
			}

			public static final Descriptor DESCRIPTOR = new Descriptor(Operands.TWO, Data.ZERO, "STMT_goto") {
				@SuppressWarnings("unchecked")
				@Override
				public Syntactic.Item construct(int opcode, Syntactic.Item[] operands, byte[] data) {
					return new Goto((Value.Int) operands[0], (Value.Int) operands[1]);
				}
			};
		}
	}

	public interface Expr extends Syntactic.Item {
		/**
		 * Represents different kinds of constants found in a given expression (e.g.
		 * string literals, integer literals, etc).
		 *
		 * @author David J. Pearce
		 *
		 */
		public static class Constant extends AbstractItem implements Expr {
			public Constant(String c) {
				this(new Value.UTF8(c));
			}
			public Constant(Value constant) {
				super(EXPR_CONST,constant);
			}

			public <T> T getAs(Class<T> kind) {
				Object constant = decode((Value) super.get(0));
				//
				if (kind.isInstance(constant)) {
					return (T) constant;
				} else {
					throw new IllegalArgumentException("invalid constant encountered");
				}
			}

			private Object decode(Value v) {
				switch(v.getOpcode()) {
				case ITEM_int:
					return ((Value.Int)v).get();
				case ITEM_utf8:
					return new String(((Value.UTF8)v).get());
				default:
					throw new IllegalArgumentException();
				}
			}

			@Override
			public Item clone(Item[] operands) {
				return new Constant((Value) operands[0]);
			}

			public static final Descriptor DESCRIPTOR = new Descriptor(Operands.ONE, Data.ZERO, "EXPR_const") {
				@SuppressWarnings("unchecked")
				@Override
				public Syntactic.Item construct(int opcode, Syntactic.Item[] operands, byte[] data) {
					return new Constant((Value) operands[0]);
				}
			};
		}
	}

	// =========================================================
	// Binary Representation
	// =========================================================

	public static final Syntactic.Schema SCHEMA;

	static {
		// Construct the schema
		SectionedSchema ROOT = new SectionedSchema(null, 1, 0, new Section[0]);
		SectionedSchema.Builder builder = ROOT.extend();
		builder.register("ITEM", 16);
		builder.register("STMT", 16);
		builder.register("EXPR", 32);
		// Items from AbstractCompilationUnit
		builder.add("ITEM", "null", AbstractCompilationUnit.Value.Null.DESCRIPTOR_0);
		builder.add("ITEM", "bool", AbstractCompilationUnit.Value.Bool.DESCRIPTOR_0);
		builder.add("ITEM", "int", AbstractCompilationUnit.Value.Int.DESCRIPTOR_0);
		builder.add("ITEM", "utf8", AbstractCompilationUnit.Value.UTF8.DESCRIPTOR_0);
		builder.add("ITEM", "pair", AbstractCompilationUnit.Pair.DESCRIPTOR_0);
		builder.add("ITEM", "tuple", AbstractCompilationUnit.Tuple.DESCRIPTOR_0);
		builder.add("ITEM", "array", AbstractCompilationUnit.Value.Array.DESCRIPTOR_0);
		builder.add("ITEM", "ident", AbstractCompilationUnit.Identifier.DESCRIPTOR_0);
		builder.add("ITEM", "name", AbstractCompilationUnit.Name.DESCRIPTOR_0);
		builder.add("ITEM", "decimal", AbstractCompilationUnit.Value.Decimal.DESCRIPTOR_0);
		builder.add("ITEM", "ref", AbstractCompilationUnit.Ref.DESCRIPTOR_0);
		builder.add("ITEM", "dictionary", AbstractCompilationUnit.Value.Dictionary.DESCRIPTOR_0);
		builder.add("ITEM", null, null);
		builder.add("ITEM", null, null);
		builder.add("ITEM", "span", AbstractCompilationUnit.Attribute.Span.DESCRIPTOR_0);
		builder.add("ITEM", "byte", AbstractCompilationUnit.Value.Byte.DESCRIPTOR_0);
		// Add statements
		builder.add("STMT", "print", Stmt.Print.DESCRIPTOR);
		builder.add("STMT", "goto", Stmt.Goto.DESCRIPTOR);
		// Add expressions
		builder.add("EXPR", "const", Expr.Constant.DESCRIPTOR);
		// Done
		SCHEMA = builder.done();
	}

	public static class Reader extends HeapReader {

		public Reader(InputStream output) {
			super(output);
		}

		@Override
		public Syntactic.Heap read() throws IOException {
			jbuildgraph.util.Pair<Integer, Syntactic.Item[]> p = readItems();
			return new BasicFile(p.first(),p.second());
		}

		@Override
		protected Syntactic.Schema checkHeader() throws IOException {
			// Currently no header for a basic file!
			return SCHEMA;
		}
	}

	public static class Writer extends HeapWriter {

		public Writer(OutputStream output) {
			super(output, SCHEMA);
		}

		@Override
		public void writeHeader() throws IOException {
			// don't do anything!
		}
	}
}
