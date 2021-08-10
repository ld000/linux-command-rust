use std::{fs, io};
use structopt::StructOpt;
use colored::{Colorize};
use std::path::PathBuf;
use std::os::macos::fs::MetadataExt;
use std::cmp::Ordering;

// Usage: ls [OPTION]... [FILE]...
// List information about the FILEs (the current directory by default).
// Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.
//
// Mandatory arguments to long options are mandatory for short options too.
//   -a, --all                  do not ignore entries starting with .
//   -A, --almost-all           do not list implied . and ..
//       --author               with -l, print the author of each file
//   -b, --escape               print C-style escapes for nongraphic characters
//       --block-size=SIZE      scale sizes by SIZE before printing them; e.g.,
//                                '--block-size=M' prints sizes in units of
//                                1,048,576 bytes; see SIZE format below
//   -B, --ignore-backups       do not list implied entries ending with ~
//   -c                         with -lt: sort by, and show, ctime (time of last
//                                modification of file status information);
//                                with -l: show ctime and sort by name;
//                                otherwise: sort by ctime, newest first
//   -C                         list entries by columns
//       --color[=WHEN]         colorize the output; WHEN can be 'never', 'auto',
//                                or 'always' (the default); more info below
//   -d, --directory            list directories themselves, not their contents
//   -D, --dired                generate output designed for Emacs' dired mode
//   -f                         do not sort, enable -aU, disable -ls --color
//   -F, --classify             append indicator (one of */=>@|) to entries
//       --file-type            likewise, except do not append '*'
//       --format=WORD          across -x, commas -m, horizontal -x, long -l,
//                                single-column -1, verbose -l, vertical -C
//       --full-time            like -l --time-style=full-iso
//   -g                         like -l, but do not list owner
//       --group-directories-first
//                              group directories before files;
//                                can be augmented with a --sort option, but any
//                                use of --sort=none (-U) disables grouping
//   -G, --no-group             in a long listing, don't print group names
//   -h, --human-readable       with -l, print sizes in human readable format
//                                (e.g., 1K 234M 2G)
//       --si                   likewise, but use powers of 1000 not 1024
//   -H, --dereference-command-line
//                              follow symbolic links listed on the command line
//       --dereference-command-line-symlink-to-dir
//                              follow each command line symbolic link
//                                that points to a directory
//       --hide=PATTERN         do not list implied entries matching shell PATTERN
//                                (overridden by -a or -A)
//       --indicator-style=WORD  append indicator with style WORD to entry names:
//                                none (default), slash (-p),
//                                file-type (--file-type), classify (-F)
//   -i, --inode                print the index number of each file
//   -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN
//   -k, --kibibytes            default to 1024-byte blocks for disk usage
//   -l                         use a long listing format
//   -L, --dereference          when showing file information for a symbolic
//                                link, show information for the file the link
//                                references rather than for the link itself
//   -m                         fill width with a comma separated list of entries
//   -n, --numeric-uid-gid      like -l, but list numeric user and group IDs
//   -N, --literal              print raw entry names (don't treat e.g. control
//                                characters specially)
//   -o                         like -l, but do not list group information
//   -p, --indicator-style=slash
//                              append / indicator to directories
//   -q, --hide-control-chars   print ? instead of nongraphic characters
//       --show-control-chars   show nongraphic characters as-is (the default,
//                                unless program is 'ls' and output is a terminal)
//   -Q, --quote-name           enclose entry names in double quotes
//       --quoting-style=WORD   use quoting style WORD for entry names:
//                                literal, locale, shell, shell-always, c, escape
//   -r, --reverse              reverse order while sorting
//   -R, --recursive            list subdirectories recursively
//   -s, --size                 print the allocated size of each file, in blocks
//   -S                         sort by file size
//       --sort=WORD            sort by WORD instead of name: none (-U), size (-S),
//                                time (-t), version (-v), extension (-X)
//       --time=WORD            with -l, show time as WORD instead of default
//                                modification time: atime or access or use (-u)
//                                ctime or status (-c); also use specified time
//                                as sort key if --sort=time
//       --time-style=STYLE     with -l, show times using style STYLE:
//                                full-iso, long-iso, iso, locale, or +FORMAT;
//                                FORMAT is interpreted like in 'date'; if FORMAT
//                                is FORMAT1<newline>FORMAT2, then FORMAT1 applies
//                                to non-recent files and FORMAT2 to recent files;
//                                if STYLE is prefixed with 'posix-', STYLE
//                                takes effect only outside the POSIX locale
//   -t                         sort by modification time, newest first
//   -T, --tabsize=COLS         assume tab stops at each COLS instead of 8
//   -u                         with -lt: sort by, and show, access time;
//                                with -l: show access time and sort by name;
//                                otherwise: sort by access time
//   -U                         do not sort; list entries in directory order
//   -v                         natural sort of (version) numbers within text
//   -w, --width=COLS           assume screen width instead of current value
//   -x                         list entries by lines instead of by columns
//   -X                         sort alphabetically by entry extension
//   -1                         list one file per line
//
// SELinux options:
//
//   --lcontext                 Display security context.   Enable -l. Lines
//                              will probably be too wide for most displays.
//   -Z, --context              Display security context so it fits on most
//                              displays.  Displays only mode, user, group,
//                              security context and file name.
//   --scontext                 Display only security context and file name.
//       --help     display this help and exit
//       --version  output version information and exit
//
// SIZE is an integer and optional unit (example: 10M is 10*1024*1024).  Units
// are K, M, G, T, P, E, Z, Y (powers of 1024) or KB, MB, ... (powers of 1000).
//
// Using color to distinguish file types is disabled both by default and
// with --color=never.  With --color=auto, ls emits color codes only when
// standard output is connected to a terminal.  The LS_COLORS environment
// variable can change the settings.  Use the dircolors command to set it.
//
// Exit status:
//  0  if OK,
//  1  if minor problems (e.g., cannot access subdirectory),
//  2  if serious trouble (e.g., cannot access command-line argument).
//
// GNU coreutils online help: <http://www.gnu.org/software/coreutils/>
// For complete documentation, run: info coreutils 'ls invocation'

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ls",
    version = "0.1.0",
    about = "List information about the FILEs (the current directory by default).
Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.",
)]
struct Opt {
    /// do not ignore entries starting with .
    #[structopt(short, long)]
    all: bool,

    /// use a long listing format
    #[structopt(short)]
    long_format: bool,
}

struct File {
    pub name: String,
    pub metadata: fs::Metadata,
    pub permissions: Permissions,
}

impl File {
    pub fn new(path: PathBuf) -> io::Result<File> {
        let name = path.file_name().unwrap().to_str().unwrap();
        let metadata = fs::symlink_metadata(&path)?;

        let mode = &metadata.st_mode();
        let has_bit = |bit| mode & bit == bit;

        let permissions = Permissions {
            user_read: has_bit(libc::S_IRUSR as u32),
            user_write: has_bit(libc::S_IWUSR as u32),
            user_execute: has_bit(libc::S_IXUSR as u32),

            group_read: has_bit(libc::S_IRGRP as u32),
            group_write: has_bit(libc::S_IWGRP as u32),
            group_execute: has_bit(libc::S_IXGRP as u32),

            other_read: has_bit(libc::S_IROTH as u32),
            other_write: has_bit(libc::S_IWOTH as u32),
            other_execute: has_bit(libc::S_IXOTH as u32),

            sticky: has_bit(libc::S_ISVTX as u32),
            setgid: has_bit(libc::S_ISGID as u32),
            setuid: has_bit(libc::S_ISUID as u32),
        };

        Ok(File{ name: name.to_string(), metadata, permissions })
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    pub fn is_hide(&self) -> bool {
        self.name.starts_with(".")
    }

    /// The number of hard links
    pub fn links(&self) -> u64 {
        self.metadata.st_nlink()
    }

    /// user ID that owns this file
    pub fn user(&self) -> u32 {
        self.metadata.st_uid()
    }

    /// group ID that owns this file
    pub fn group(&self) -> u32 {
        self.metadata.st_gid()
    }
}

impl Eq for File {}

impl PartialEq<Self> for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd<Self> for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

struct Permissions {
    user_read: bool,
    user_write: bool,
    user_execute: bool,

    group_read: bool,
    group_write: bool,
    group_execute: bool,

    other_read: bool,
    other_write: bool,
    other_execute: bool,

    sticky: bool,
    setgid: bool,
    setuid: bool,
}

fn main() -> io::Result<()> {
    let opt : Opt = Opt::from_args();

    println!("{:?}", opt);

    let mut file_vec: Vec<File> = Vec::new();
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        file_vec.push(File::new(path.unwrap().path())?);
    }

    file_vec.sort();

    let format = |b, show| if b { show } else { "-" };
    let format_name = |is_dir, name: String| if is_dir { name.bright_cyan() } else { name.normal() };

    for file in file_vec {
        if file.is_hide() && !opt.all {
            continue;
        }

        if opt.long_format {
            println!("{}{}{}{}{}{}{}{}{}{} {} {} {} {}",
                     format(file.is_dir(), "d"),
                     format(file.permissions.user_read, "r"),
                     format(file.permissions.user_write, "w"),
                     format(file.permissions.user_execute, "x"),
                     format(file.permissions.group_read, "r"),
                     format(file.permissions.group_write, "w"),
                     format(file.permissions.group_execute, "x"),
                     format(file.permissions.other_read, "r"),
                     format(file.permissions.other_write, "w"),
                     format(file.permissions.other_execute, "x"),
                     file.links(),
                     users::get_user_by_uid(file.user()).unwrap().name().to_str().unwrap(),
                     users::get_group_by_gid(file.group()).unwrap().name().to_str().unwrap(),
                     format_name(file.is_dir(), file.name)
            );
        } else {
            print!("{} ", format_name(file.is_dir(), file.name));
        }
    }

    Ok(())
}
