package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

import jbfs.core.Build.Artifact;
import jbfs.core.Content;
import jbfs.util.Trie;

public class SourceFile extends jbfs.core.SourceFile {
	public static Content.Type<SourceFile> ContentType = new Content.Type<>() {

		@Override
		public String getSuffix() {
			return "basic";
		}

		@Override
		public SourceFile read(Trie id, InputStream input, Registry registry) throws IOException {
			return new SourceFile(id, new String(input.readAllBytes()));
		}

		@Override
		public void write(OutputStream output, SourceFile value) throws IOException {
			throw new UnsupportedOperationException();
		}

	};

	public SourceFile(Trie id, String content) {
		super(id, content);
	}

	@Override
	public Type<? extends Artifact> getContentType() {
		return ContentType;
	}
}
