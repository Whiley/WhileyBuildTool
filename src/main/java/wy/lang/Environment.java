package wy.lang;

import java.util.Iterator;

import jcmdarg.core.Command;
import wy.commands.Root;
import jbuildgraph.core.Build;
import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Trie;
import jbuildstore.core.Content;
import jbuildstore.util.HashMapStore;

public class Environment {
	/**
	 * Plugin environment provides access to information sourced from the plugins,
	 * such as available content-types, commands, etc.
	 */
	private final Plugin.Environment env;
	/**
	 * Root command hierarchy for the tool.
	 */
	private final Root root;
	/**
	 * The main repository for storing build artifacts and source files which is
	 * properly versioned.
	 */
	private final Content.Store<Trie, Artifact> repository;
	/**
	 * The working directory where build artifacts are projected, etc.
	 */
	private final Content.Store<Trie, Content> workingRoot;

	public Environment(Plugin.Environment env, Iterable<Artifact> entries, Content.Store<Trie, Content> workingRoot) {
		this.env = env;
		this.root = new Root(env);
		this.repository = new HashMapStore<>();
		this.workingRoot = workingRoot;
		// Initialise store
		for (Artifact b : entries) {
			repository.put(b.getPath(), b);
		}
	}

	/**
	 * Get the list of registered commands.
	 *
	 * @return
	 */
	public Iterable<Command.Descriptor<Environment,Boolean>> getCommandDescriptors() {
		return (Iterable) env.getAll(Command.Descriptor.class);
	}

	/**
	 * Get the list of registered content types.
	 * @return
	 */
	public Iterable<Content.Type<? extends Content>> getContentTypes() {
		return (Iterable) env.getAll(Content.Type.class);
	}

	/**
	 * Get the list of registered build platforms.
	 * @return
	 */
	public Iterable<Build.Platform<?>> getBuildPlatforms() {
		return (Iterable) env.getAll(Build.Platform.class);
	}

	public Command.Descriptor<Environment, Boolean> getRootDescriptor() {
		return root;
	}
}
