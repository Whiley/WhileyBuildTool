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
package hello.world;

import java.util.Arrays;
import java.util.List;
import jcmdarg.core.Command;
import jcmdarg.core.Command.Arguments;
import jcmdarg.core.Option.Descriptor;
import wy.lang.Environment;
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
	public static final Command.Descriptor<Environment,Boolean> COMMAND = new Command.Descriptor<>() {

		@Override
		public String getName() {
			return "hello";
		}

		@Override
		public String getDescription() {
			return "The simplest possible command";
		}

		@Override
		public List<Descriptor> getOptionDescriptors() {
			return Arrays.asList();
		}

		@Override
		public Command<Boolean> initialise(Environment env) {
			return new Command<>() {
				@Override
				public Boolean execute() {
					// Do something
					System.out.println("world");
					// Always succeeds!
					return true;
				}
			};
		}

		@Override
		public Environment apply(Arguments<Environment, Boolean> instance, Environment env) {
			// No arguments are supported!
			return env;
		}

		@Override
		public List<Command.Descriptor<Environment, Boolean>> getCommands() {
			// No subcommands
			return Arrays.asList();
		}

	};

	@Override
	public Plugin start(Context context) {
		context.logTimedMessage("[Hello World] starting!", 0,0);
		// Register platform
		context.register(Command.Descriptor.class, COMMAND);
		//
		return new Plugin() {

		};
	}

	@Override
	public void stop(Plugin module, Context context) {
		context.logTimedMessage("[Hello World] finishing", 0,0);
	}
}
