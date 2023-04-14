# cpu_usage-polybar
A small script in rust to get the cpu usage in %'s with a gradient color for the text


Inside polybar settings;
```
[module/cpus_usage]
type = custom/script
exec = ~/.config/polybar/scripts/cpu_usage.sh
tail = true

# Optional past this line
format-prefix-foreground = ${colors.primary}
format-prefix = CPU's: (
format-suffix = )
format-suffix-foreground = ${colors.primary}
label = %output%
```


and the cpu_usage.sh:
```
#!/bin/bash

~/Rusty/cpu_usage/target/release/cpu_usage
