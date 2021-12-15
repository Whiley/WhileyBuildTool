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
package concat;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.List;

import jbuildgraph.core.Build;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Key;
import jbuildstore.util.TextFile;
import wy.lang.Plugin;
import wy.lang.Plugin.Context;

/**
 * This illustrates the simplest possible plugin for the Wy build tool. It adds
 * a simple command "hello" which prints "world" (no surprises there).
 *
 * @author David J. Pearce
 *
 */
public class Activator implements Plugin.Activator {
	/**
	 * The build platform is responsible for initialising the concat task within a
	 * given environment. It is registered with the build system so that it can be
	 * used when `wy build` is executed.
	 */
	public Build.Platform<String> PLATFORM = new Build.Platform<>() {

		@Override
		public Task initialise(String context) {
			System.out.println("Initialise concat platform");
			return new Task();
		}

	};

	@Override
	public Plugin start(Context context) {
		context.logTimedMessage("[Concat] starting!", 0, 0);
		// Register ASCII as default encoding of ".txt" files.
		context.register(Content.Type.class, TextFile.ContentTypeASCII);
		// Register build platform
		context.register(Build.Platform.class, PLATFORM);
		//
		return new Plugin() {

		};
	}

	@Override
	public void stop(Plugin module, Context context) {
		context.logTimedMessage("[Concat] finishing", 0,0);
	}

	public static class Task implements Build.Task {
		private static Key<Trie, ?> TARGET_ID = new Key<>(Trie.fromString("output.txt"), TextFile.ContentTypeASCII);

		@Override
		public boolean apply(Content.Store<Key<Trie, ?>> repository) {
			// Match all source files
			try {
				List<? extends TextFile> files = repository.getAll(k -> k.contentType() == TextFile.ContentTypeASCII ? (Key<Trie,TextFile>) k : null);
				// Generate their concatenation
				StringBuffer result = new StringBuffer();
				for (TextFile tf : files) {
					// FIXME: this is a bit dumb
					String s = new String(tf.getBytes(StandardCharsets.US_ASCII));
					result.append(s);
				}
				repository.put(TARGET_ID, new TextFile(TextFile.ContentTypeASCII, result.toString()));
				// Write out the dump
				return true;
			} catch (IOException e) {
				// TODO Auto-generated catch block
				e.printStackTrace();
				return false;
			}
		}
	}
}
