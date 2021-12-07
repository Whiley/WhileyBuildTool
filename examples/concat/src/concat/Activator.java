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

import java.util.List;

import jbuildgraph.core.Build;
import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content.Ledger;
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
			return new Task();
		}

	};

	@Override
	public Plugin start(Context context) {
		context.logTimedMessage("[Hello World] starting!", 0, 0);
		// Register build platform
		context.register(Build.Platform.class, PLATFORM);
		//
		return new Plugin() {

		};
	}

	@Override
	public void stop(Plugin module, Context context) {
		context.logTimedMessage("[Hello World] finishing", 0,0);
	}

	public static class Task implements Build.Task {

		@Override
		public List<Artifact> requires() {
			// TODO Auto-generated method stub
			return null;
		}

		@Override
		public List<Artifact> ensures() {
			// TODO Auto-generated method stub
			return null;
		}

		@Override
		public boolean apply(Ledger<Trie, Artifact> repository) {
			// TODO Auto-generated method stub
			return false;
		}

	}
}
