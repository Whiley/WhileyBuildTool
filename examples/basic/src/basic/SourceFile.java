package basic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.util.TextFile;


public class SourceFile extends TextFile {
	public static Content.Type<SourceFile> ContentType = new Content.Type<>() {

		@Override
		public SourceFile read(InputStream input) throws IOException {
			// Read all bytes from input stream
			byte[] bytes = input.readAllBytes();
			// Convert them into a string
			return new SourceFile(new String(bytes, StandardCharsets.US_ASCII));
		}

		@Override
		public void write(OutputStream output, SourceFile value) throws IOException {
			// Extract bytes from text file
			byte[] bytes = value.getBytes(StandardCharsets.US_ASCII);
			// Write them to output stream
			output.write(bytes);
		}

		@Override
		public String suffix() {
			return "basic";
		}
	};

	public SourceFile(String content) {
		super(ContentType, content);
	}

	@Override
	public Content.Type<SourceFile> contentType() {
		return ContentType;
	}
}
