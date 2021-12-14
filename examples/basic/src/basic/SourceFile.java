package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.util.TextFile;


public class SourceFile extends jbuildstore.util.TextFile {
	public static Content.Type<SourceFile> ContentType = new Content.Type<>() {

		@Override
		public SourceFile read(InputStream input) throws IOException {
			// TODO Auto-generated method stub
			return null;
		}

		@Override
		public void write(OutputStream output, SourceFile value) throws IOException {
			// TODO Auto-generated method stub

		}

		@Override
		public String suffix() {
			// TODO Auto-generated method stub
			return null;
		}

//		@Override
//		public String getSuffix() {
//			return "basic";
//		}
//
//		@Override
//		public SourceFile read(Trie id, InputStream input, Registry registry) throws IOException {
//			return new SourceFile(id, new String(input.readAllBytes()));
//		}
//
//		@Override
//		public void write(OutputStream output, SourceFile value) throws IOException {
//			throw new UnsupportedOperationException();
//		}

	};

	public SourceFile(Trie id, String content) {
		super(content);
	}

	@Override
	public Content.Type<TextFile> contentType() {
		// This doesn't make sense to me!
		return super.ContentTypeASCII;
	}
}
