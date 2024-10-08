## goseis

<div>
  <img src="https://img.shields.io/github/last-commit/nahuelmol/rustserver"/>
  <img src="https://img.shields.io/github/languages/code-size/nahuelmol/rustserver"/>
  <img src="https://img.shields.io/github/languages/top/nahuelmol/rustserver"/>
</div>

possible command line tool for processing .segy data. From unprocessed data:
* generate CMP gather
* perform normal moveout correction
* perform dip moveout correction
* get the semblance

## commands


+ goseis startproject: starts a new project
+ goseis projects: checks for projects
+ goseis current: checks for the current project
+ goseis sw <project>: switch to the indicated project

+ goseis load <file.segy> : just add a file to the project
+ goseis display: visualize files
+ goseis dt <trace_number>:display trace
+ goseis delete <filename.segy>: delete a file from the workflow. I must specify its state, as an unprocessed file or anything else

