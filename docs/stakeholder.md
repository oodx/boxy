# Stakeholder Requirements

This document outlines any incomplete concepts, tools, verifications or patterns
needed to properly complete the project. 

FEAT: labels call out new work that needs to be added, other lines are validations, tests, or concept elaborations

## 1. Terminal Width
- FEAT: width should support --width [num|max|auto]
- boxy should end at the terminal width $COLUMNS
  - ``` 
    width=$(tput cols 2>/dev/null || stty size 2>/dev/null | awk '{print $2}' || echo 80); echo "$width
    ```
    - width validation tests 
      - `tput cols </dev/tty`
      - `stty size </dev/tty | awk '{print $2}'`
      - `echo "$COLUMNS"`
- FEAT: boxy needs a `width` command to debug width issues



```bash
# wrong implementation
┌────────── UAT Test Result ───────────┐
│ ✅                                   │
│ Test passed                          │
└──────────────────────────────────────┘

# this is the correct placement of title
┌──────────────────────────────────────┐
│ ✅ UAT Test Result                   │
│ Test passed                          │
└──────────────────────────────────────┘
```

Both Title and Status are implemented incorrectly

```bash
# this is correct implementation of header and title
┌─────────────── Header ───────────────┐
│ ✅ UAT Test Result                   │
│ Test passed                          │
└─────────────── Footer ───────────────┘


# wrong status implementation
┌──────────────────────────────────────┐
│ status                               │
└─────────────── Footer ───────────────┘
hello!

```


```bash

┌─────────────── Header ───────────────┐
│ @ UAT Test Result                    │
├──────────────────────────────────────┤ #dt
│ Test passed                          │
│ Test passed                          │
├──────────────────────────────────────┤ #ds
│ status                               │
└─────────────── Footer ───────────────┘


┌─────────────── Header ───────────────┐
│ @ UAT Test Result                    │
├──────────────────────────────────────┤ #dtn
│                                      │
│ Test passed                          │
│ Test passed                          │
│                                      │ #dsn
├──────────────────────────────────────┤
│ status                               │
└─────────────── Footer ───────────────┘

```

```
┌─────────────── Header ───────────────┐
│                                      │ #stn
│ @ UAT Test Result                    │
│                                      │ #ptn (pad)
│ Test passed                          │
│ Test passed                          │ 
│                                      │ #psn (pad)
│ status                               │
│                                      │ #ssn
└─────────────── Footer ───────────────┘


```
# MAJOR ENHANCEMENT, expanding the two-letter layout names but for content streams
#streaming format support uses k='v'; streams
echo "hd='this is my head label'; \
      ft='this is my footlabel';  \
      st='my status is';  \
      bd='all my body are belong to us /n';" \
| boxy --theme silly --width max
as a flag
--param="hd='this is my head label'; \
          ft='this is my footlabel';  \
          st='my status is';  \
          bd='all my body are belong to us /n';"


echo "Test passed" | boxy --header "Header" --title "@ UAT Test Result" --layout 'stn,ptn,psn,ssn' --status status --footer Footer --width 40

echo "all my body are belong to us" | boxy --content "hd='Header'; tl='Title';  st='status'; ft='Footer'; ic='@'" --width max

echo "This is the main body content" | \
boxy --theme success --title "Title" \
--header "Header" \
--status "Status" --width 40 \
--layout "hl"

echo "Test passed" | boxy --header "Header" --title "UAT Test Result" --status status --footer Footer --layout 'hl,fc,sc' --width max

33         horizontal: "─", vertical: "│",
    34    +    tee_left: "├", tee_right: "┤", cross: "┼",
    35     };
    ⋮
    40         horizontal: "─", vertical: "│",
    41    +    tee_left: "├", tee_right: "┤", cross: "┼",
    42     };
    ⋮
    47         horizontal: "═", vertical: "║",
    48    +    tee_left: "╠", tee_right: "╣", cross: "╬",
    49     };
    ⋮
    54         horizontal: "━", vertical: "┃",
    55    +    tee_left: "┣", tee_right: "┫", cross: "╋",
    56     };
    ⋮
    61         horizontal: "-", vertical: "|",
    62    +    tee_left: "+", tee_right: "+", cross: "+",
