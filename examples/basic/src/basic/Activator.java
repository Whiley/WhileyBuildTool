package basic;

import java.io.IOException;
import java.util.List;

import jbuildsled.core.Build;
import jbuildsled.core.Content;
import jbuildsled.util.Trie;
import wy.cfg.Configuration;
import wy.cfg.Configuration.Schema;
import wy.lang.Command;
import wy.lang.Plugin;
import wy.lang.Command.Environment;
import wy.lang.Plugin.Context;

public class Activator implements Plugin.Activator {

	public static final Command.Platform BASIC_PLATFORM = new Command.Platform() {

		@Override
		public String getName() {
			return "basic";
		}

		@Override
		public Schema getConfigurationSchema() {
			return Configuration.EMPTY_SCHEMA;
		}

		@Override
		public Build.Task initialise(Trie path, Environment environment) throws IOException {
			// Search snapshot for all source files
			Build.SnapShot snapshot = environment.getRepository().last();
			Trie srcdir = Trie.fromString("src");
			Content.Filter<SourceFile> includes = Content.Filter(SourceFile.ContentType,
					srcdir.append(Trie.EVERYTHING));
			List<SourceFile> files = snapshot.getAll(includes);
			//
			System.out.println("FOUND FILES: " + files + " FROM: " + snapshot);
			//
			return new CompileTask(path, files.get(0));
		}

	};

	@Override
	public Plugin start(Context context) {
		System.out.println("BASIC PLUGIN STARTING!");
		// Register platform
		context.register(Command.Platform.class, BASIC_PLATFORM);
		// List of content types
		context.register(Content.Type.class, SourceFile.ContentType);
		context.register(Content.Type.class, BinaryFile.ContentType);
		//
		return new Plugin() {

		};
	}

	@Override
	public void stop(Plugin module, Context context) {
		System.out.println("BASIC PLUGIN FINISHING!");
	}

}
