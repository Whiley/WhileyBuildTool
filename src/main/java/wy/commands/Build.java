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

import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import jbuildgraph.core.Build.*;
import jbuildgraph.core.SourceFile;
import jbuildgraph.core.Build.Artifact;
import jbuildgraph.util.Trie;
import jcmdarg.core.Command;
import jcmdarg.core.Option;
import jcmdarg.util.Options;
import wy.lang.Environment;
import wy.lang.Syntactic;

/**
 *
 * @author David J. Pearce
 *
 */
public class Build implements Command<Boolean> {
	public static Trie BUILD_PLATFORMS = Trie.fromString("build/platforms");
	/**
	 * The descriptor for this command.
	 */
	public static final Command.Descriptor<Environment, Boolean> DESCRIPTOR = new Command.Descriptor<>() {
		@Override
		public String getName() {
			return "build";
		}

		@Override
		public String getDescription() {
			return "Perform build operations on an existing project";
		}

		@Override
		public List<Option.Descriptor> getOptionDescriptors() {
			return Arrays.asList(Options.FLAG("verbose", "generate verbose information about the build", false),
					Options.FLAG("brief", "generate brief output for syntax errors", false));
		}

		@Override
		public List<Descriptor<Environment, Boolean>> getCommands() {
			return Collections.emptyList();
		}

		@Override
		public Build initialise(Environment environment) {
			return new Build(environment, System.out, System.err);
		}

		@Override
		public Environment apply(Arguments<Environment, Boolean> instance, Environment state) {
			return state;
		}
	};

	/**
	 * Provides a generic place to which normal output should be directed. This
	 * should eventually be replaced.
	 */
	private final PrintStream sysout;

	/**
	 * Provides a generic place to which error output should be directed. This
	 * should eventually be replaced.
	 */
	private final PrintStream syserr;

	/**
	 * Signals that brief error reporting should be used. This is primarily used to
	 * help integration with external tools. More specifically, brief output is
	 * structured so as to be machine readable.
	 */
	protected boolean brief = false;

	/**
	 * The enclosing project for this build
	 */
	private final Environment environment;

	public Build(Environment environment, OutputStream sysout, OutputStream syserr) {
		this.environment = environment;
		this.sysout = new PrintStream(sysout);
		this.syserr = new PrintStream(syserr);
	}

	@Override
	public Boolean execute() {
		System.out.println("[build] execute");
		// Initialise all build platforms
		List<Platform<?>> platforms = determineBuildPipeline();
		System.out.println("[build] initialise");
		// Initialise the build pipeline
		List<Task> tasks = initialisePipeline(platforms);
		System.out.println("[build] run");
		// Execute pipeline sequentially (for now)
		for(Task task : tasks) {
			if(!task.apply(null)) {
				// Someone went wrong
				return false;
			}
		}
		// Success!
		return true;
	}

	private List<Platform<?>> determineBuildPipeline() {
		ArrayList<Platform<?>> platforms = new ArrayList<>();
		environment.getBuildPlatforms().forEach(platforms::add);
		// FIXME: need to intersect list of active platforms
		return platforms;
	}

	private List<Task> initialisePipeline(List<Platform<?>> platforms) {
		ArrayList<Task> tasks = new ArrayList<>();
		for(Platform<?> p : platforms) {
			// FIXME: what goes here?
			tasks.add(p.initialise(null));
		}
		//
		return tasks;
	}

	/**
	 * Print out syntactic markers for all entries in the build graph. This requires
	 * going through all entries, extracting the markers and then printing them.
	 *
	 * @param executor
	 * @throws IOException
	 */
	public static void printSyntacticMarkers(PrintStream output, Artifact target) throws IOException {
		// Extract all syntactic markers from entries in the build graph
		List<Syntactic.Marker> items = extractSyntacticMarkers(target);
		// For each marker, print out error messages appropriately
		for (int i = 0; i != items.size(); ++i) {
			// Log the error message
			printSyntacticMarkers(output, items.get(i), target.getSourceArtifacts());
		}
	}

	public static void printSyntacticMarkers(PrintStream output, Syntactic.Marker marker,
			List<? extends Artifact> sources) {
//		// Identify enclosing source file
//		SourceFile source = getSourceEntry(marker.getSource(), sources);
//		String filename = source.getPath().toString();
//		//
//		Span span = marker.getTarget().getAncestor(AbstractCompilationUnit.Attribute.Span.class);
//		// Read the enclosing line so we can print it
//		SourceFile.Line line = source.getEnclosingLine(span.getStart().get().intValue());
//		// Sanity check we found it
//		if (line != null) {
//			// print the error message
//			output.println(filename + ":" + line.getNumber() + ": " + marker.getMessage());
//			// Finally print the line highlight
//			printLineHighlight(output, span, line);
//		} else {
//			output.println(filename + ":?: " + marker.getMessage());
//		}
	}

	public static List<Syntactic.Marker> extractSyntacticMarkers(Artifact... binaries) throws IOException {
		List<Syntactic.Marker> annotated = new ArrayList<>();
		//
		for (Artifact b : binaries) {
			// If the object in question can be decoded as a syntactic heap then we can look
			// for syntactic messages.
			if (b instanceof Syntactic.Heap) {
				annotated.addAll(extractSyntacticMarkers((Syntactic.Heap) b));
			}
		}
		//
		return annotated;
	}

	/**
	 * Traverse the various binaries which have been generated looking for error
	 * messages.
	 *
	 * @param binaries
	 * @return
	 * @throws IOException
	 */
	public static List<Syntactic.Marker> extractSyntacticMarkers(Syntactic.Heap h) throws IOException {
		throw new IllegalArgumentException();
	}

	private static void printLineHighlight(PrintStream output, Syntactic.Span span, SourceFile.Line enclosing) {
		// Extract line text
		String text = enclosing.getText();
		// Determine start and end of span
		int start = span.getStart() - enclosing.getOffset();
		int end = Math.min(text.length() - 1, span.getEnd() - enclosing.getOffset());
		// NOTE: in the following lines I don't print characters
		// individually. The reason for this is that it messes up the
		// ANT task output.
		output.println(text);
		// First, mirror indendation
		String str = "";
		for (int i = 0; i < start; ++i) {
			if (text.charAt(i) == '\t') {
				str += "\t";
			} else {
				str += " ";
			}
		}
		// Second, place highlights
		for (int i = start; i <= end; ++i) {
			str += "^";
		}
		output.println(str);
	}
}
