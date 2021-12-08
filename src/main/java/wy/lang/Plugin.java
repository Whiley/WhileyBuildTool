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
package wy.lang;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;

import jbuildgraph.core.Build;
import jbuildstore.core.Content;
import jcmdarg.core.Command;
import wy.util.Logger;

/**
 * <p>
 * A module describes a collection of one or more features which can be deployed
 * within a running system (for example, though not exclusively, the Whiley
 * Compiler Collection).
 * </p>
 *
 * @author David J. Pearce
 *
 */
public interface Plugin {

	/**
	 * A module Context provides a mechanism for modules to interact with their
	 * environment. In particular, it allows them to register extension points which
	 * provide the critical mechanism for adding new functionality.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface Context extends Logger {

		/**
		 * Responsible for registering a feature as implementing an extension within the
		 * system.
		 *
		 * @param ep        The class representing the extension point (e.g.
		 *                  "wyfs.ContentType").
		 * @param extension The implementation of the given extension point.
		 */
		public <T> void register(Class<T> ep, T extension);

		/**
		 * Create a new extension point which subsequent modules can register extensions
		 * for. This employs a default implementation of <code>ExtensionPoint</code>
		 * which is backed by an <code>ArrayList</code>.
		 *
		 * @param extension
		 * @param ep
		 */
		public <T> void create(Class<T> extension);

		/**
		 * Create a new extension point which subsequent modules can register extensions
		 * for.
		 *
		 * @param extension
		 * @param ep
		 */
		public <T> void create(Class<T> extension, ExtensionPoint<T> ep);
	}

	/**
	 * An extension point in the module is a named entity provided by one module,
	 * which other modules can register extensions for.
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface ExtensionPoint<T> extends Iterable<T> {
		/**
		 * Notify extension point that a new extension has been registered for it.
		 *
		 * @param extension The extension implementation to register with this extension
		 *                  point.
		 */
		public void register(T extension);
	}

	/**
	 * Represents a class designated as the unique "activator" for a given module.
	 * This activator is used to control aspects of the module (e.g. resources
	 * allocated) as it is started and stopped,
	 *
	 * @author David J. Pearce
	 *
	 */
	public interface Activator {

		/**
		 * This method is called when the module is begun. This gives the module an
		 * opportunity to register one or more extension points in the compiler.
		 *
		 * @param context
		 */
		public Plugin start(Context context);

		/**
		 * This method is called when the module is stopped. Any resources used by the
		 * module should be freed at this point. This includes any registered extension
		 * points, which should be unregistered.
		 *
		 * @param context
		 */
		public void stop(Plugin module, Context context);
	}

	/**
	 * Provides a default plugin environment which is generally sufficient.
	 */
	public static class Environment implements Plugin.Context {
		/**
		 * Logging stream, which is null by default.
		 */
		private Logger logger = Logger.NULL;

		/**
		 * The extension points represent registered implementations of interfaces. Each
		 * extension point represents a class that will be instantiated and configured,
		 * and will contribute to some function within the compiler. The main extension
		 * points are: <i>Routes</i>, <i>Builders</i> and <i>ContentTypes</i>.
		 */
		public final HashMap<Class<?>, ExtensionPoint<?>> extensionPoints = new HashMap<>();

		@SuppressWarnings("rawtypes")
		public Environment(Logger logger) {
			this.logger = logger;
		}

		@SuppressWarnings("unchecked")
		public <T> Iterable<T> getAll(Class<T> kind) {
			return (Iterable<T>) extensionPoints.get(kind);
		}

		public void setLogger(Logger logger) {
			this.logger = logger;
		}

		/**
		 * Activate a new plugin within the system.
		 *
		 * @param activator
		 */
		public void activate(wy.lang.Plugin.Activator activator) {
			Plugin p = activator.start(this);
			// NOTE: there is quite a lot more we could do here.
		}

		// ==================================================================
		// Context Methods
		// ==================================================================

		@Override
		public <T> void register(Class<T> kind, T extension) {
			@SuppressWarnings("unchecked")
			ExtensionPoint<T> ep = (ExtensionPoint<T>) extensionPoints.get(kind);
			if (ep == null) {
				throw new RuntimeException("Missing extension point: " + kind.getCanonicalName());
			} else {
				ep.register(extension);
			}
		}

		@Override
		public <T> void create(Class<T> extension) {
			this.create(extension, new ExtensionPointList<T>());
		}

		@Override
		public <T> void create(Class<T> extension, wy.lang.Plugin.ExtensionPoint<T> ep) {
			if (extensionPoints.containsKey(extension)) {
				throw new RuntimeException("Extension point already exists: " + extension);
			} else {
				extensionPoints.put(extension, ep);
			}
		}

		@Override
		public void logTimedMessage(String msg, long time, long memory) {
			logger.logTimedMessage(msg, time, memory);
		}
	}

	/**
	 * A simple extension point which is backed by an <code>ArrayList</code>.
	 *
	 * @author David J. Pearce
	 *
	 * @param <T>
	 */
	public static class ExtensionPointList<T> implements ExtensionPoint<T> {
		private final ArrayList<T> extensions = new ArrayList<>();
		@Override
		public Iterator<T> iterator() {
			return extensions.iterator();
		}

		@Override
		public void register(T extension) {
			extensions.add(extension);
		}
	}
}