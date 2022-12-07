use std::collections::HashMap;
use std::env;
use std::fs;
/*
--- Day 7: No Space Left On Device ---
You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally,
you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to
run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output
(your puzzle input). For example:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

The filesystem consists of a tree of files (plain data) and directories (which can contain other
directories or files). The outermost directory is called /. You can navigate around the filesystem,
moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some
modern computers:

cd means change directory. This changes which directory is the current directory, but the specific
result depends on the argument:

cd x moves in one level: it looks in the current directory for the directory named x and makes it
the current directory.

cd .. moves out one level: it finds the directory that contains the current directory, then makes
that directory the current directory.

cd / switches the current directory to the outermost directory, /.

ls means list. It prints out all of the files and directories immediately contained by the current
directory:

123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.

Given the commands and output in the example above, you can determine that the filesystem looks
visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

Here, there are four directories: / (the outermost directory), a and d (which are in /), and e
(which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good
candidates for deletion. To do this, you need to determine the total size of each directory. The
total size of a directory is the sum of the sizes of the files it contains, directly or indirectly.
(Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other
directories.

The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and
h.lst (size 62596), plus file i indirectly (a contains e which contains i).

Directory d has total size 24933642.

As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size
of every file.

To begin, find all of the directories with a total size of at most 100000, then calculate the sum
of their total sizes. In the example above, these directories are a and e; the sum of their total
sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes
of those directories?


*/

struct State {
    cwd: std::path::PathBuf,
    // keeps total file size in each dir, including subdirs (not including subdirs!)
    dirs: HashMap<String, u64>,

    // keeps total file size in each dir, including subdirs (including subdirs!)
    dirs_with_subdirs: HashMap<String, u64>,
}

impl State {
    fn handle_command(&mut self, command: &str) {
        if command.starts_with("cd") {
            let (_, path) = command.split_once(" ").unwrap();

            match path.trim() {
                ".." => {
                    println!(
                        "In {}, call cd-up",
                        self.cwd.clone().into_os_string().into_string().unwrap()
                    );
                    self.cwd.pop();
                }
                "/" => {
                    println!(
                        "In {}, call cd /",
                        self.cwd.clone().into_os_string().into_string().unwrap()
                    );
                    self.cwd = std::path::PathBuf::from("/");
                }
                _ => {
                    // actualy cd
                    println!(
                        "In {}, call cd {}",
                        self.cwd.clone().into_os_string().into_string().unwrap(),
                        path.trim(),
                    );
                    self.cwd.push(path.trim());
                }
            }
        } else if command == "ls" {
            // nothin to do
            return;
        }
    }

    fn handle_output(&mut self, line: &str) {
        // this only applies to ls
        if line.starts_with("dir") {
            let mut d = self.cwd.clone();
            d.push(line[3..].trim());
            let d_str: String = d.into_os_string().into_string().unwrap();
            println!(
                "In {}, found {}",
                self.cwd.clone().into_os_string().into_string().unwrap(),
                &d_str,
            );
            self.dirs.entry(d_str).or_insert(0);
        } else {
            // file size, add to total directory size, we don't actually
            // care about the file name
            let (file_size_s, _) = line.split_once(" ").unwrap();

            let file_size: u64 = file_size_s.parse::<u64>().unwrap();
            println!(
                "In {}, found size: {}",
                self.cwd.clone().into_os_string().into_string().unwrap(),
                file_size,
            );

            // add to current working directory and every parent directory
            let mut d = self.cwd.clone();
            while d != std::path::Path::new("/") {
                let d_str: String = d.clone().into_os_string().into_string().unwrap();
                self.dirs_with_subdirs
                    .entry(d_str)
                    .and_modify(|v| *v += file_size)
                    .or_insert(file_size);
                d.pop();
            }
        }
    }

    fn handle_line(&mut self, line: &str) {
        if line.starts_with("$") {
            self.handle_command(line[1..].trim());
        } else {
            self.handle_output(line);
        }
    }
}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    let mut state = State {
        cwd: std::path::PathBuf::from("/"),
        dirs: Default::default(),
        dirs_with_subdirs: Default::default(),
    };
    for line in contents.lines() {
        state.handle_line(line.trim());
    }

    println!("Direct");
    for (path, size) in state.dirs {
        println!("{}: {}", path, size);
    }

    println!("Full");
    let mut sum1 = 0; // total under 100000

    // find smallest directory with at least
    // 8_381_165 bytes to delete
    let mut smallest_over = std::u64::MAX;
    for (path, size) in state.dirs_with_subdirs {
        println!("{}: {}", path, size);
        if size <= 100000 {
            sum1 += size;
        }

        if size > 8_381_165 && size < smallest_over {
            smallest_over = size;
        }
    }
    println!("Result1: {}", sum1);
    println!("Result2: {}", smallest_over);
}
