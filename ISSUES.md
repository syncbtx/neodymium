## Neodymium

### 1. Align attached + islands bars with niri windows: `unverified`
- niri puts `gaps` between a window and a layer-shell zone, so the island-to-window distance is inner padding + niri gap.
- **Rule:** inner-facing padding = `max(0, padding - niri_gap)`, outer-facing padding unchanged. Inner side = bottom for a top bar, top for bottom, right for left, left for right.
- **Code:** `bar.rs` (`trim_inner_padding`); apply it for attached + islands bars. The zone shrinks automatically, because `bar_cross` derives from the padding.
- **techinicalities** need to efficiently keep niri gaps and neodymium paddings consistent, which is easier if niri gaps 
  flowed directly into the paddings uniformly for a unified look
- **Open:** a floating bar's inner inset needs the same trim. The corner where two bars meet is untested.

### 2. Configuring niri from the shell: `workaround`
- niri's IPC has no request for config values and no gaps action. There is a `load-config-file` action (seen only in a third-party skill listing; check `niri msg action --help`).
- **Workaround:** the shell writes `amane.kdl` (`layout { gaps N }`), and the user adds `include "amane.kdl"` at the end of `config.kdl`. Needs niri 25.11+, and niri live-reloads. Don't generate `struts`, because they are not merged.
- **Open:** the gap value must live in the model and be passed into `view()`.

## Resolved

- **Floating "stopped applying" after the `bar_padding` refactor:** the attachment mode hadn't been toggled. The envelope and insets were fine.
