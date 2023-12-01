add package:
  #!/usr/bin/env bash
  md {{package}}
  cd {{invocation_directory()}}
  cargo new {{package}}

