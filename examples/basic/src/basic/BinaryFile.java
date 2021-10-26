package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import jbfs.core.Build;
import jbfs.core.Build.Artifact;
import jbfs.core.Content;
import jbfs.util.Trie;

public class BinaryFile implements Build.Artifact {

	public static Content.Type<BinaryFile> ContentType = new Content.Type<>() {

		@Override
		public String getSuffix() {
			return "bil";
		}

		@Override
		public BinaryFile read(Trie id, InputStream input, Registry registry) throws IOException {
			throw new UnsupportedOperationException();
		}

		@Override
		public void write(OutputStream output, BinaryFile value) throws IOException {
			throw new UnsupportedOperationException();
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
	public Trie getPath() {
		return path;
	}

	@Override
	public Type<? extends Artifact> getContentType() {
		return ContentType;
	}

	@Override
	public List<? extends Artifact> getSourceArtifacts() {
		return Arrays.asList(source);
	}

	// =========================================================
	// Statements
	// =========================================================

	public interface Stmt {

	}
}
