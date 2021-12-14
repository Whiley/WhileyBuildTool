package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;


public class BinaryFile implements Content {

	public static Content.Type<BinaryFile> ContentType = new Content.Type<>() {

		@Override
		public BinaryFile read(InputStream input) throws IOException {
			throw new UnsupportedOperationException();
		}

		@Override
		public void write(OutputStream output, BinaryFile value) throws IOException {
			throw new UnsupportedOperationException();
		}

		@Override
		public String suffix() {
			return "bil";
		}


	};

	private final Trie path;
	private final SourceFile source;
	private final ArrayList<Stmt> stmts;

	public BinaryFile(Trie path, SourceFile source, List<Stmt> stmts) {
		this.path = path;
		this.source = source;
		this.stmts = new ArrayList<>();
	}

	@Override
	public Type<BinaryFile> contentType() {
		return ContentType;
	}

	// =========================================================
	// Statements
	// =========================================================

	public interface Stmt {

	}
}
