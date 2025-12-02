<details class="pokemon-card-container">
<summary>Shuppet (#127)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-shuppet">
<input type="radio" name="pokemon-tabs-shuppet-group" id="pokemon-tabs-shuppet-tab-0" checked>
<label for="pokemon-tabs-shuppet-tab-0">Shuppet</label>
<input type="radio" name="pokemon-tabs-shuppet-group" id="pokemon-tabs-shuppet-tab-1">
<label for="pokemon-tabs-shuppet-tab-1">Banette</label>
<input type="radio" name="pokemon-tabs-shuppet-group" id="pokemon-tabs-shuppet-tab-2">
<label for="pokemon-tabs-shuppet-tab-2">Mega Banette</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-shuppet-panel-0">
Types: Ghost / Normal • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Insomnia
- Frisk
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Held Item**
Spell Tag

**Encounter Locations**
- Nyx Trails — Grass (Night) (10%)
- Pythios Cemetery — Grass (Day) (8%)
- Wanderer's Woods (North) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="shuppet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">54</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">63</span> |
| Sp. Def | <span class="stat-value stat-low">43</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">325</span> |

**Level-Up Moves**
- Knock Off (Lv 1)
- Screech (Lv 4)
- Night Shade (Lv 7)
- Spite (Lv 10)
- Shadow Sneak (Lv 13)
- Will-O-Wisp (Lv 16)
- Feint Attack (Lv 19)
- Headbutt (Lv 21)
- Hex (Lv 22)
- Curse (Lv 26)
- Glare (Lv 28)
- Shadow Ball (Lv 30)
- Embargo (Lv 34)
- Body Slam (Lv 36)
- Sucker Punch (Lv 38)
- Darkest Lariat (Lv 38)
- Rage Fist (Lv 40)
- Snatch (Lv 42)
- Grudge (Lv 46)
- Trick (Lv 50)
- Bitter Blade (Lv 52)
- Phantom Force (Lv 54)

**Egg Moves**
- Disable
- Destiny Bond
- Foresight
- Astonish
- Imprison
- Pursuit
- Shadow Sneak
- Confuse Ray
- Ominous Wind
- Gunk Shot
- Phantom Force

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
- Metronome
- Psych Up
- Sleep Talk
- Snore
- Swagger
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-shuppet-panel-1">
Types: Ghost / Normal • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Insomnia
- Frisk
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Held Item**
Spell Tag

**Evolution Info**
Lv. 28

**Encounter Locations**
- Nyx Trails — Grass (Day) (4%)
- Nyx Trails — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="banette" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">74</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-mid">53</span> |
| Sp. Def | <span class="stat-value stat-mid">73</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">465</span> |

**Level-Up Moves**
- Darkest Lariat (Lv Evo)
- Phantom Force (Lv 1)
- Knock Off (Lv 1)
- Screech (Lv 4)
- Night Shade (Lv 7)
- Spite (Lv 10)
- Shadow Sneak (Lv 13)
- Will-O-Wisp (Lv 16)
- Feint Attack (Lv 19)
- Headbutt (Lv 21)
- Hex (Lv 22)
- Curse (Lv 26)
- Glare (Lv 28)
- Shadow Ball (Lv 30)
- Embargo (Lv 34)
- Body Slam (Lv 36)
- Sucker Punch (Lv 38)
- Rage Fist (Lv 40)
- Snatch (Lv 42)
- Grudge (Lv 46)
- Trick (Lv 50)
- Bitter Blade (Lv 52)
- Phantom Force (Lv 54)

**Egg Moves**
- Disable
- Destiny Bond
- Foresight
- Astonish
- Imprison
- Pursuit
- Shadow Sneak
- Confuse Ray
- Ominous Wind
- Gunk Shot
- Phantom Force

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
- Metronome
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swords Dance
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-shuppet-panel-2">
Types: Ghost / Dark • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Prankster

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Psychic (0×)

*Weak to*
- Fairy (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Held Item**
Spell Tag

**Evolution Info**
Banettite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-banette" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">74</span> |
| Attack | <span class="stat-value stat-high">165</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">93</span> |
| Sp. Def | <span class="stat-value stat-mid">83</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-high">565</span> |

**Level-Up Moves**
- Darkest Lariat (Lv Evo)
- Phantom Force (Lv 1)
- Knock Off (Lv 1)
- Screech (Lv 4)
- Night Shade (Lv 7)
- Spite (Lv 10)
- Shadow Sneak (Lv 13)
- Will-O-Wisp (Lv 16)
- Feint Attack (Lv 19)
- Headbutt (Lv 21)
- Hex (Lv 22)
- Curse (Lv 26)
- Glare (Lv 28)
- Shadow Ball (Lv 30)
- Embargo (Lv 34)
- Body Slam (Lv 36)
- Sucker Punch (Lv 38)
- Rage Fist (Lv 40)
- Snatch (Lv 42)
- Grudge (Lv 46)
- Trick (Lv 50)
- Bitter Blade (Lv 52)
- Phantom Force (Lv 54)

**Egg Moves**
- Disable
- Destiny Bond
- Foresight
- Astonish
- Imprison
- Pursuit
- Shadow Sneak
- Confuse Ray
- Ominous Wind
- Gunk Shot
- Phantom Force

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
- Metronome
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swords Dance
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
</div>
</div>
<style>
#pokemon-tabs-shuppet-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-shuppet-panel-0 { display: block; }
#pokemon-tabs-shuppet-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-shuppet-panel-1 { display: block; }
#pokemon-tabs-shuppet-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-shuppet-panel-2 { display: block; }
</style>
</details>
