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
import java.util.Arrays;
import java.util.List;

import jbuildstore.core.Content;
import jcmdarg.core.Command;
import jcmdarg.core.Command.Arguments;
import jcmdarg.core.Option.Descriptor;

import wy.cfg.Configuration;
import wy.cfg.Configuration.Schema;
import wy.lang.Plugin;
import wy.lang.Environment;
import wy.lang.Plugin.Context;

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
			throw new UnsupportedOperationException();
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
		System.out.println("BASIC PLUGIN STARTING!");
		// Register platform
		//context.register(Command.Platform.class, BASIC_PLATFORM);
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
