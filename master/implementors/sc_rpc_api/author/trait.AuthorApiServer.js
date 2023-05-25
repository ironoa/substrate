(function() {var implementors = {
"sc_rpc":[["impl&lt;P, Client&gt; <a class=\"trait\" href=\"sc_rpc/author/trait.AuthorApiServer.html\" title=\"trait sc_rpc::author::AuthorApiServer\">AuthorApiServer</a>&lt;&lt;P as TransactionPool&gt;::Hash, &lt;&lt;P as TransactionPool&gt;::Block as <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Hash\" title=\"type sp_runtime::traits::Block::Hash\">Hash</a>&gt; for <a class=\"struct\" href=\"sc_rpc/author/struct.Author.html\" title=\"struct sc_rpc::author::Author\">Author</a>&lt;P, Client&gt;<span class=\"where fmt-newline\">where\n    P: TransactionPool + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    Client: HeaderBackend&lt;P::Block&gt; + ProvideRuntimeApi&lt;P::Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    Client::Api: SessionKeys&lt;P::Block&gt;,\n    P::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,\n    &lt;P::Block as <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Hash\" title=\"type sp_runtime::traits::Block::Hash\">Hash</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()