// Copyright 2021 The Whiley Project Developers
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
package basic;

import java.io.IOException;
import java.util.List;

import jcmdarg.core.Command;
import jbuildgraph.core.Build;
import jbuildstore.core.Content;
import jbuildgraph.util.Trie;
import wy.cfg.Configuration;
import wy.cfg.Configuration.Schema;
import wy.lang.Plugin;
import wy.lang.Environment;
import wy.lang.Plugin.Context;

public class Activator implements Plugin.Activator {

	public static final Command.Descriptor<String,Boolean> BASIC_PLATFORM = new Command.Descriptor<>() {

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
