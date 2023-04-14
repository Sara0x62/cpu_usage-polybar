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
```
![2023-04-14_14-54](https://user-images.githubusercontent.com/83826811/232077169-ec326e28-fc02-42c3-a517-5397941406c5.png)
![2023-04-14_15-00](https://user-images.githubusercontent.com/83826811/232077174-b75d83ed-de11-46c6-839e-e41ce079bcf4.png)
