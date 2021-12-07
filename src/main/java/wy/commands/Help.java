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

import java.io.PrintStream;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import jbuildgraph.util.Trie;
import jcmdarg.core.Command;
import jcmdarg.core.Option;
import jcmdarg.util.Options;
import wy.cfg.Configuration;
import wy.lang.Environment;


public class Help implements Command<Boolean> {

    public static final Configuration.Schema SCHEMA = Configuration
            .fromArray(Configuration.BOUND_INTEGER(Trie.fromString("width"), "fix display width", false, 0));

    public static final List<Option.Descriptor> OPTIONS = Arrays
            .asList(Options.UNSIGNED_INTEGER("width", "fix display width", 80));

    /**
     * The descriptor for this command.
     */
    public static final Descriptor<Environment,Boolean> DESCRIPTOR = new Descriptor<>() {
        @Override
        public String getName() {
            return "help";
        }

        @Override
        public String getDescription() {
            return "Display help information";
        }

        @Override
        public List<Option.Descriptor> getOptionDescriptors() {
            return OPTIONS;
        }

		@Override
		public Command<Boolean> initialise(Environment state) {
			return new Help(System.out,state);
		}

		@Override
		public Environment apply(Arguments<Environment, Boolean> instance, Environment state) {
			return state;
		}

		@Override
		public List<Descriptor<Environment, Boolean>> getCommands() {
			return Collections.emptyList();
		}
    };
    //
    private final PrintStream out;
    private final Environment environment;

    public Help(PrintStream out, Environment environment) {
        this.environment = environment;
        this.out = out;
    }

    @Override
    public Boolean execute() {
    	printUsage();
        //
        return true;
    }

    public static void print(PrintStream out, Descriptor<Environment,Boolean> descriptor) {
        out.println("NAME");
        out.println("\t" + descriptor.getName());
        out.println();
        out.println("DESCRIPTION");
        out.println("\t" + descriptor.getDescription());
        out.println();
        out.println("OPTIONS");
        List<Option.Descriptor> options = descriptor.getOptionDescriptors();
        for (int i = 0; i != options.size(); ++i) {
            Option.Descriptor option = options.get(i);
            String argument = option.getArgumentDescription();
            out.print("\t--" + option.getName());
            if(argument != null && !argument.equals("")) {
                out.print("=" + argument);
            }
            out.println();
            out.println("\t\t" + option.getDescription());
        }
        out.println();
        out.println("SUBCOMMANDS");
		List<Descriptor<Environment, Boolean>> commands = descriptor.getCommands();
        for (int i = 0; i != commands.size(); ++i) {
			Descriptor<Environment, Boolean> d = commands.get(i);
            out.println("\t" + d.getName());
            out.println("\t\t" + d.getDescription());
        }
        out.println();
//        out.println("CONFIGURATION");
//        Configuration.Schema schema = descriptor.getConfigurationSchema();
//        List<Configuration.KeyValueDescriptor<?>> descriptors = schema.getDescriptors();
//        for (int i = 0; i != descriptors.size(); ++i) {
//            Configuration.KeyValueDescriptor<?> option = descriptors.get(i);
//            out.println("\t" + option.getFilter());
//            out.println("\t\t" + option.getDescription());
//        }
    }

    /**
     * Print usage information to the console.
     */
	protected void printUsage() {
		out.println("usage: wy [--verbose] command [<options>] [<args>]");
		out.println();
		int maxWidth = determineCommandNameWidth();
		out.println("Commands:");
		for (Descriptor d : environment.getCommandDescriptors()) {
			out.print("  ");
			out.print(rightPad(d.getName(), maxWidth));
			out.println("   " + d.getDescription());
		}
		out.println();
		out.println("Run `wy help COMMAND` for more information on a command");
	}

    /**
     * Right pad a given string with spaces to ensure the resulting string is
     * exactly n characters wide. This assumes the given string has at most n
     * characters already.
     *
     * @param str
     *            String to right-pad
     * @param n
     *            Width of resulting string
     * @return
     */
    public static String rightPad(String str, int n) {
        return String.format("%1$-" + n + "s", str);
    }

    /**
     * Left pad a given string with spaces to ensure the resulting string is
     * exactly n characters wide. This assumes the given string has at most n
     * characters already.  No, this is not its own library.
     *
     * @param str
     *            String to left-pad
     * @param n
     *            Width of resulting string
     * @return
     */
    public static String leftPad(String str, int n) {
        return String.format("%1$" + n + "s", str);
    }

    /**
     * Determine the maximum width of any configured command name
     *
     * @param descriptors
     * @return
     */
	private int determineCommandNameWidth() {
        int max = 0;
        for (Descriptor<Environment,Boolean> d : environment.getCommandDescriptors()) {
            max = Math.max(max, d.getName().length());
        }
        return max;
    }
}
