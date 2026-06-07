This is a real step up. The screens now feel like they belong to the same game instead of eight cousins wearing borrowed panel borders. The main thing left is not “more polish,” it is **more compression**.

Your current pattern still tends to do this:

* show list item
* show selected card
* show status of the selected card
* show a sentence that repeats the status
* show an empty panel telling us it is empty

That is where most duplication and dead weight is coming from.

## Global rules to apply to every screen

These will remove a lot of overlap fast:

**1. Never show the same fact in more than 2 places**

* Example: selected room name in list, art caption, panel title, and action preview is too much.
* Pick: list label + detail title.

**2. Empty panels should collapse**

* If a section has nothing in it, turn it into a one-line empty state or hide it entirely.

**3. Status should live in one consistent place**

* Use a single status pill/strip near the title of the selected item.
* Do not repeat status in body text and footer hints.

**4. Action bars should only show relevant actions**

* If someone is already Idle, do not show a big Idle button next to an Idle status.
* Contextual buttons beat permanent button clutter.

**5. Summary metrics should be chips/cards, not mini paragraphs**

* Gold, risk, eggs, debt, availability, count, source, grade all work best as compact chips.

**6. Reserve full-width text for one thing only**

* Either flavor/explainer text, or actionable guidance.
* Not both on the same screen.

---

# Screen-by-screen feedback

## 1. Main Menu

### What is working

* Good central focal block
* The art panel helps break up the text
* Button hierarchy is much clearer now

### What is duplicated / weak

* The giant empty top utility bar is mostly just a runway for the Settings button
* The title is doing a lot of work, while the subtitle text is a bit too long and airy
* The background tower silhouette is nice, but it competes slightly with the right-side art panel

### Improve it like this

* Shrink the top bar height by about 30 to 40%
* Move Settings into a small icon/button anchored tighter to the top-right
* Tighten the body copy to 2 lines, not 4
* Make the right art panel a little narrower, so the left text and primary CTA dominate
* Increase spacing between the title and the copy, but reduce copy width

### Remove / compress

* Remove any extra framing that is not helping the focal panel
* Keep just:

  * Title
  * 2-line premise
  * 3 buttons
  * art card

---

## 2. Town Overview

### What is working

* “Today’s Priority” is the right idea
* Snapshot cards are much clearer than before
* Footer navigation is strong and consistent

### What is duplicated / weak

* The **Chamber** action appears in both Today’s Priority and the footer
* “Town: Campaign loaded from save” is not useful permanent information
* The roster card repeats state:

  * Idle tag
  * Idle button
  * character card context already implies current state
* Huge empty space to the right of the roster feels like unused UI territory

### Improve it like this

* Remove the “campaign loaded from save” strip entirely
* Keep the top-left priority panel focused on:

  * one sentence
  * one CTA
* In the roster card:

  * show name, species, role/state, key stats
  * hide one of the Idle indicators
* If only one girl is present, center or widen the card instead of leaving a giant dead zone
* Consider turning the footer button that matches the priority into a highlighted tab, instead of duplicating a separate CTA button

### Remove / compress

* Remove the save-state strip
* Do not show both current state and same-state action
* Collapse empty roster space with a responsive card layout

---

## 3. Town Management

### What is working

* Much cleaner than earlier versions
* Left list + selected building + support panels is a sound structure
* Availability icons in the list are useful

### What is duplicated / weak

* Availability is shown too many times:

  * icon in list
  * “Available” status
  * green “build now” text
  * active Build button
* “Next Destination: Return To Town” does not feel useful enough to deserve a full row
* Effects and Progression Web both feel like support panels competing for attention
* Some building names are truncated in the list, which makes the left rail feel cramped

### Improve it like this

* Keep availability in just two places:

  * small list icon
  * detail status pill
* Replace “Build now. The cost is covered…” with a short affordance chip:

  * `Affordable`
* Turn cost, category, and built count into a single horizontal chip row
* Replace “Next Destination” with actual unlock chips or a useful follow-up action
* Widen the left list slightly or reduce font size only for long building names
* Merge Progression Web into a smaller strip if it is not interactive

### Remove / compress

* Remove the full “build now” sentence
* Remove “Next Destination” unless it becomes actionable
* Reduce lower support area to one dominant support panel, not two equal ones

---

## 4. GuildJobs Management

### What is working

* Big improvement from the older version
* Splitting “Assigned Here” and “Available” is cleaner
* The selected room detail panel is easier to parse now

### What is duplicated / weak

* Room name appears in:

  * list item
  * art caption
  * detail title
* Status “Room plan ready” is fine, but projected value is weak when it says `0 gold / 0 lust`
* Empty Assigned Here panel is too large for saying “nothing here”
* Worker card is cramped:

  * portrait
  * name
  * species
  * projection
  * 3 buttons
* Buttons are too close to the text block and feel squashed

### Improve it like this

* Remove the room name from the art caption
* Keep the room title only once in the detail header
* If projected output is zero because nobody is assigned, show:

  * `No workers assigned`
    instead of a dead-value projection
* Collapse empty Assigned Here into a short box, maybe 80 to 100px tall
* Rebuild the worker card layout into:

  * portrait
  * name/species
  * one projected earnings line
  * action buttons on a fixed-width right rail
* If only one available worker exists, center the card or widen it to avoid lonely-card syndrome

### Remove / compress

* Remove duplicate room naming
* Replace zero-value projections with actionable empty messaging
* Collapse the empty Assigned Here panel

---

## 5. Chamber Management

### What is working

* This screen is one of the strongest structurally
* Egg inventory on the left and selected egg details on the right is good
* The hatch CTA is clear

### What is duplicated / weak

* Source appears both in the top status row and again in the selected egg details
* “Locked Outcome” appears multiple times
* Grade/common and potential outcomes are repeated between inventory cards and selected detail
* With only one possible outcome, the lower section feels too large and too empty

### Improve it like this

* Use the top bar for chamber-level info only:

  * egg count
  * maybe source summary
* Use selected detail for egg-specific info only
* Show “Locked Outcome” once, as a pill under the egg name
* If only one outcome exists, make it a compact result card instead of a huge wide block
* Move hatch cost into the button area:

  * `Hatch (45 gold / 6 lust)`
* The inventory cards can be simplified to:

  * egg thumbnail
  * rarity
  * one-line outcome state
  * Review button

### Remove / compress

* Remove duplicate source info
* Remove repeated “Locked Outcome” labels
* Shrink the possible outcome area when outcome count is one

---

## 6. Expedition Planning

### What is working

* Much clearer than the previous pass
* Metrics row is useful
* Mission and priority controls are visually grouped better now

### What is duplicated / weak

* Floor name appears in list, image caption, and detail title
* “Status: Plan expedition” is not pulling its weight
* Too many metrics are given equal emphasis, including zero-value ones
* Team card repeats state awkwardly:

  * Idle tag
  * Idle button
* Action buttons feel too close to the character info

### Improve it like this

* Remove the floor name from the art caption
* Remove or repurpose the status strip into something useful like:

  * `1 member assigned`
  * `Risk: Low`
* Hide zero metrics unless they matter

  * `Projected Eggs 0` and `Projected Relics 0` do not need equal weight
* Highlight only:

  * Success
  * Main reward
  * Injury risk
* Rebuild the expedition member card into:

  * portrait
  * name / role
  * current state
  * right-aligned action buttons
* If current state is Idle, show Assign + Rest, not Idle + Idle-state chip

### Remove / compress

* Remove duplicate floor naming
* Remove dead status text
* Hide non-important zero metrics

---

## 7. Guest Management

### What is working

* The structure is clean
* The right-side summary cards are easier to read than before
* The screen has a good top-level hierarchy when content exists

### What is duplicated / weak

Right now the empty state is repeated three times:

* No Requests
* No Request Selected
* No active request selected

This is the biggest duplication problem in the whole set.

### Improve it like this

When there are no requests:

* Collapse the Selected Request panel
* Collapse or hide Eligible Girls
* Turn the screen into a single meaningful empty state:

  * `No active requests today`
  * `Check again tomorrow or build demand sources in town`
* Keep just a compact summary strip on the right if needed

When requests do exist:

* Request list on left
* Selected request center
* Eligible girls below or right
* Summary cards in a thin top row, not a big side chunk

### Remove / compress

* Never show two empty panels explaining the same absence
* Collapse Eligible Girls entirely when no request is selected
* Move the economy summary to a slimmer header row

---

## 8. Settings

### What is working

* Modal treatment is good
* Resolution buttons are understandable
* Save feedback appearing inline is a good step

### What is duplicated / weak

* `Fullscreen: Off` duplicates the active display mode below
* Display mode block is larger than it needs to be
* Three near-identical settings states make the screen feel static
* Quit Game inside Settings is risky and visually mixed with utility actions
* Save Campaign and Close are fine, but the lower button row feels a little too spread out

### Improve it like this

* Remove the `Fullscreen: Off` row entirely
* Let the mode selector carry the state
* Make resolution tiles a 2-column grid with clearer selected styling
* Move Save feedback directly above or inside the Save button zone, which you already started doing
* Separate destructive navigation:

  * utility row: Save, Close
  * destructive row or confirm dialog: Quit Game
* Tighten vertical spacing between Display Mode and Resolution

### Remove / compress

* Remove the duplicate fullscreen state row
* Consider moving Quit Game out of settings unless it is truly needed there

---

# Biggest duplication offenders across the whole UI

These are the recurring culprits:

## 1. Name duplication

Common pattern:

* list label
* art caption
* selected title

Keep only:

* list label
* selected title

## 2. Empty state duplication

Common pattern:

* one panel says nothing selected
* another panel says nothing selected
* a third panel says nothing exists

Collapse empties into one message.

## 3. State duplication

Common pattern:

* current state chip
* same-state button
* descriptive sentence repeating that state

Keep only:

* state chip
* contextual action buttons

## 4. Availability duplication

Common pattern:

* icon
* status text
* affordance sentence
* enabled button

Keep only:

* icon or status pill
* enabled button

---

# Where overlaps and crowding are most likely to happen next

These are the screens most at risk as content grows:

### GuildJobs Management

Worker cards will start colliding first.
Fix with a fixed action column and fewer text lines.

### Expedition Planning

Metric cards and selector buttons will crowd fast.
Fix with priority tiers and hiding zero/secondary metrics.

### Town Management

Long building names and lower support panels will start fighting.
Fix with wider list rail or shorter display names.

### Guest Management

Once requests exist, right-side summary plus selected request plus eligible roster may become a three-way pileup.
Fix by slimming summary cards into a top strip.

---

# My recommended next-pass priority

Do these first:

1. **Collapse duplicate empty states**
2. **Remove art-caption titles when a panel title already exists**
3. **Remove same-state action buttons**
4. **Convert repeated availability text into a single pill**
5. **Tighten bottom action rows so only relevant actions remain**

That will make the UI feel cleaner fast, without needing a full redesign.

If you want, I can turn this into a **screen-by-screen wireframe rewrite**, where I describe the exact sections each screen should contain and what should be removed.
