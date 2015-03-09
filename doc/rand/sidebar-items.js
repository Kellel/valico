initSidebarItems({"fn":[["random","Generates a random value using the thread-local random number generator."],["sample","Randomly sample up to `amount` elements from an iterator."],["thread_rng","Retrieve the lazily-initialized thread-local random number generator, seeded by the system. Intended to be used in method chaining style, e.g. `thread_rng().gen::<i32>()`."],["weak_rng","Create a weak random number generator with a default algorithm and seed."]],"mod":[["chacha","The ChaCha random number generator."],["distributions","Sampling from random distributions."],["isaac","The ISAAC random number generator."],["os","Interfaces to the operating system provided random number generators."],["reader","A wrapper around any Reader to treat it as an RNG."],["reseeding","A wrapper around another RNG that reseeds it after it generates a certain number of random bytes."]],"struct":[["AsciiGenerator","Iterator which will continuously generate random ascii characters."],["Closed01","A wrapper for generating floating point numbers uniformly in the closed interval `[0,1]` (including both endpoints)."],["Generator","Iterator which will generate a stream of random items."],["Open01","A wrapper for generating floating point numbers uniformly in the open interval `(0,1)` (not including either endpoint)."],["OsRng","A random number generator that retrieves randomness straight from the operating system. Platform sources:"],["StdRng","The standard RNG. This is designed to be efficient on the current platform."],["ThreadRng","The thread-local RNG."],["XorShiftRng","An Xorshift[1] random number generator."]],"trait":[["Rand","A type that can be randomly generated using an `Rng`."],["Rng","A random number generator."],["SeedableRng","A random number generator that can be explicitly seeded to produce the same stream of randomness multiple times."]]});