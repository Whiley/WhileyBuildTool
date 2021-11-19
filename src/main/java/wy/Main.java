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
package wy;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;
import java.util.*;

import jbuildgraph.core.Build;
import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Pair;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.core.Content.Type;
import jbuildstore.core.Key;
import jbuildstore.util.HashMapStore;
import jbuildstore.util.DirectoryStore;
import jbuildstore.util.ZipFile;
import jcmdarg.core.Command;
import jcmdarg.core.Command.Arguments;
import jcmdarg.core.Option;
import jcmdarg.util.Options;
import wy.cfg.*;
import wy.cfg.Configuration.Schema;
import wy.commands.*;
import wy.lang.Environment;
import wy.lang.Plugin;
import wy.lang.Syntactic;
import wy.util.Logger;
import wy.util.SuffixRegistry;

/**
 * Provides a command-line interface to the Whiley Compiler Collection. This is
 * responsible for various tasks, such as loading various configuration files
 * from disk, activating plugins, parsing command-line arguments and actually
 * activating the tool itself.
 *
 * @author David J. Pearce
 *
 */
public class Main {

	@SuppressWarnings("unchecked")
	public static final List<Command.Descriptor<Environment, Boolean>> commands = new ArrayList<>() {{
		add(BuildCmd.DESCRIPTOR);
		add(HelpCmd.DESCRIPTOR);
	}};

	/**
	 * Root descriptor for the tool.
	 */
	public static final Command.Descriptor<Environment, Boolean> DESCRIPTOR = new Command.Descriptor<>() {

		public List<Option.Descriptor> getOptionDescriptors() {
			return Arrays.asList(
					Options.FLAG("verbose", "generate verbose information about the build", false),
					Options.FLAG("brief", "generate brief output for syntax errors", false));
		}

		public String getName() {
			return null;
		}

		public String getDescription() {
			return "The Whiley Build Tool";
		}

		public List<Command.Descriptor<Environment, Boolean>> getCommands() {
			return commands;
		}

		public Command<Boolean> initialise(Environment env) {
			return () -> true;
		}

		public Environment apply(Arguments<Environment, Boolean> args, Environment env) {
			return env;
		}
	};

	// ==================================================================
	// Main Method
	// ==================================================================

	public static void main(String[] args) throws Exception {
		Logger logger = BOOT_LOGGER;
		SuffixRegistry<Content> registry = new SuffixRegistry<>();
		// Determine system-wide directory. This contains configuration relevant to the
		// entire ecosystem, such as the set of active plugins.
		DirectoryStore<Trie, Content> SystemDir = determineSystemRoot();
		// Read the system configuration file
		Configuration system = readConfigFile(SystemDir, Trie.fromString("wy"), logger, Schemas.SYSTEM_CONFIG_SCHEMA);
		// Construct plugin environment and activate plugins
		Plugin.Environment penv = activatePlugins(system, logger);
		// Register all content types defined by plugins
		registry.addAll(penv.getContentTypes());
		// Register content type for configuration files
		registry.add(ConfigFile.ContentType);
		// Register all plugin commands
		commands.addAll(penv.getCommandDescriptors());
		// Determine top-level directory and relative path
		Pair<File, Trie> lrp = determineLocalRootDirectory();
		File localDir = lrp.first();
		Trie path = lrp.second();
		// Construct working directory
		DirectoryStore<Trie, Content> workingDir = new DirectoryStore<>(registry, localDir);
		// Extract build artifacts
		List<Build.Artifact> artifacts = new ArrayList<>();
		for (Content content : workingDir) {
			if (content instanceof Build.Artifact) {
				artifacts.add((Build.Artifact) content);
			}
		}
		// Construct command environment!
		Environment env = new Environment(penv, artifacts, workingDir);
		// Execute the given command
		int exitCode = exec(env, path, args);
		// Done
		System.exit(exitCode);
	}

	public static int exec(Environment env, Trie path, String[] _args) {
		// Parse command-line arguments
		Command.Arguments<Environment, Boolean> args = Command.parse(DESCRIPTOR, _args);
		// Extract top-level arguments (if any)
		boolean verbose = args.getOptions().get(Boolean.class, "verbose");
		// Done
		try {
			// Initialise command and execute it!
			Boolean b = args.initialise(env).execute();
			// Done
			return b ? 0 : 1;
		} catch (Syntactic.Exception e) {
			e.outputSourceError(System.err, false);
			if (verbose) {
				printStackTrace(System.err, e);
			}
			return 1;
		} catch (Exception e) {
			System.err.println("Internal failure: " + e.getMessage());
			if (verbose) {
				e.printStackTrace();
			}
			return 2;
		}
	}

	// ==================================================================
	// Helpers
	// ==================================================================

	/**
	 * Determine the system root. That is, the installation directory for the
	 * compiler itself.
	 *
	 * @param tool
	 * @return
	 * @throws IOException
	 */
	private static DirectoryStore<Trie, Content> determineSystemRoot() throws IOException {
		String whileyhome = System.getenv("WHILEYHOME");
		if (whileyhome == null) {
			System.err.println("error: WHILEYHOME environment variable not set");
			System.exit(-1);
		}
		return new DirectoryStore<>(BOOT_REGISTRY, new File(whileyhome));
	}

	/**
	 * Determine where the root of this project is. This is the nearest enclosing
	 * directory containing a "wy.toml" file. The point is that we may be operating
	 * in some subdirectory of the project and want the tool to automatically search
	 * out the real root for us.
	 *
	 * @return
	 * @throws IOException
	 */
	private static Pair<File, Trie> determineLocalRootDirectory() throws IOException {
		// Search for inner configuration.
		File inner = findConfigFile(new File("."));
		if (inner == null) {
			return new Pair<>(new File("."), Trie.ROOT);
		}
		// Search for enclosing configuration (if applicable).
		File outer = findConfigFile(inner.getParentFile());
		if (outer == null) {
			// No enclosing configuration found.
			return new Pair<>(inner, Trie.ROOT);
		} else {
			// Calculate relative path
			String path = inner.getPath().replace(outer.getPath(), "").replace(File.separatorChar, '/');
			// Done
			return new Pair<>(outer, Trie.fromString(path));
		}
	}

	/**
	 * Activate the set of registed plugins which the tool uses. Currently this list
	 * is statically determined, but eventually it will be possible to dynamically
	 * add plugins to the system.
	 *
	 * @param verbose
	 * @param locations
	 * @return
	 */
	private static Plugin.Environment activatePlugins(Configuration global, Logger logger) {
		Plugin.Environment env = new Plugin.Environment(logger);
		// Determine the set of install plugins
		List<Trie> plugins = global.matchAll(Trie.fromString("plugins/*"));
		// start modules
		for (Trie id : plugins) {
			String activator = global.get(String.class, id);
			// Only activate if enabled
			try {
				Class<?> c = Class.forName(activator.toString());
				Plugin.Activator instance = (Plugin.Activator) c.newInstance();
				env.activate(instance);
			} catch (ClassNotFoundException e) {
				e.printStackTrace();
			} catch (InstantiationException e) {
				e.printStackTrace();
			} catch (IllegalAccessException e) {
				e.printStackTrace();
			}
		}
		// Done
		return env;
	}

	private static File findConfigFile(File dir) {
		// Traverse up the directory hierarchy
		while (dir != null && dir.exists() && dir.isDirectory()) {
			File wyf = new File(dir + File.separator + "wy.toml");
			if (wyf.exists()) {
				return dir;
			}
			// Traverse back up the directory hierarchy looking for a suitable directory.
			dir = dir.getParentFile();
		}
		// If we get here then it means we didn't find a root, therefore just use
		// current directory.
		return null;
	}

	/**
	 * Used for reading the various configuration files prior to instantiating the
	 * main tool itself.
	 */
	public static SuffixRegistry<Content> BOOT_REGISTRY = new SuffixRegistry<>() {
		{
			add(ConfigFile.ContentType);
		}
	};

	/**
	 * Simple default logger
	 */
	public static Logger BOOT_LOGGER = new Logger.Default(System.err);

	/**
	 * Attempt to read a configuration file from a given root.
	 *
	 * @param name
	 * @param root
	 * @return
	 * @throws IOException
	 */
	public static Configuration readConfigFile(DirectoryStore<Trie, Content> root, Trie id, Logger logger,
			Configuration.Schema... schemas) throws IOException {
		// Combine schemas together
		Configuration.Schema schema = Configuration.toCombinedSchema(schemas);
		try {
			// Read the configuration file
			ConfigFile cf = root.get(ConfigFile.ContentType, id);
			// Sanity check we found something
			if (cf == null) {
				logger.logTimedMessage("Not found " + root.getDirectory() + "/" + id + ".toml", 0, 0);
				return Configuration.EMPTY(schema);
			} else {
				// Log the event
				logger.logTimedMessage("Read " + root.getDirectory() + "/" + id + ".toml", 0, 0);
				// Construct configuration according to given schema
				return cf.toConfiguration(schema, false);
			}
		} catch (Syntactic.Exception e) {
			e.outputSourceError(System.out, false);
			System.exit(-1);
			return null;
		}
	}

	/**
	 * Print a complete stack trace. This differs from Throwable.printStackTrace()
	 * in that it always prints all of the trace.
	 *
	 * @param out
	 * @param err
	 */
	private static void printStackTrace(PrintStream out, Throwable err) {
		out.println(err.getClass().getName() + ": " + err.getMessage());
		for (StackTraceElement ste : err.getStackTrace()) {
			out.println("\tat " + ste.toString());
		}
		if (err.getCause() != null) {
			out.print("Caused by: ");
			printStackTrace(out, err.getCause());
		}
	}
}
