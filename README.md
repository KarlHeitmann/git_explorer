# TODO
Change this project so I can pass a commit hash, it will found the commit on the repository, and then I can start navigating the children and parent commits -starting from that commit- by hitting the arrow keys. Present the diffs and all the relevant details in widgets on the same view/window. Only draw the graph as long as I am hitting the arrow keys.

This can be very useful if I found a commit with `git log -p <file_name>` and I want to know what happened around that commit.

It is not relevant to draw at once the complete git graph. Because there are already a lot of tools that do it. But I haven't found anything yet that can quickly bring you to a specific place in history, present you all the context around that change, and as long as you need it show you more context.

# Screenshot

Current state of the art
![image](https://github.com/KarlHeitmann/git_explorer/assets/3003032/da17c7e3-19e6-41c4-b0ff-59dd9239b8e3)

