// Copyright 2011 The Whiley Project Developers
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
package wy.commands;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import jcmdarg.core.Command;
import jcmdarg.core.Option;
import jcmdarg.core.Command.Arguments;
import jcmdarg.util.Options;
import wy.lang.Environment;
import wy.lang.Plugin;

/**
 * Root descriptor for the tool.
 */
public class RootCmd implements Command.Descriptor<Environment, Boolean> {
	private final Plugin.Environment env;

	public RootCmd(Plugin.Environment env) {
		this.env = env;
	}

	@Override
	public List<Option.Descriptor> getOptionDescriptors() {
		return Arrays.asList(
				Options.FLAG("verbose", "generate verbose information about the build", false),
				Options.FLAG("brief", "generate brief output for syntax errors", false));
	}

	@Override
	public String getName() {
		return null;
	}

	@Override
	public String getDescription() {
		return "The Whiley Build Tool";
	}

	@SuppressWarnings("unchecked")
	@Override
	public List<Command.Descriptor<Environment, Boolean>> getCommands() {
		ArrayList result = new ArrayList<>();
		env.getAll(Command.Descriptor.class).forEach(result::add);
		return result;
	}

	@Override
	public Command<Boolean> initialise(Environment env) {
		return () -> true;
	}

	@Override
	public Environment apply(Arguments<Environment, Boolean> args, Environment env) {
		// FIXME: should do something with the arguments
		return env;
	}
};