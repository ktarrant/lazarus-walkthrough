<details class="pokemon-card-container">
<summary>Panpour (#032)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-panpour">
<input type="radio" name="pokemon-tabs-panpour-group" id="pokemon-tabs-panpour-tab-0" checked>
<label for="pokemon-tabs-panpour-tab-0">Panpour</label>
<input type="radio" name="pokemon-tabs-panpour-group" id="pokemon-tabs-panpour-tab-1">
<label for="pokemon-tabs-panpour-tab-1">Simipour</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-panpour-panel-0">
Types: Water / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Gluttony
- Cud Chew
- Torrent *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM28 - Dig
- TM32 - Double Team
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- HM01 - Cut
- HM03 - Surf
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Acrisia City — Grass (Day) (5%)
- Acrisia City — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="panpour" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">58</span> |
| Defense | <span class="stat-value stat-mid">53</span> |
| Sp. Atk | <span class="stat-value stat-mid">58</span> |
| Sp. Def | <span class="stat-value stat-mid">53</span> |
| Speed | <span class="stat-value stat-mid">64</span> |
| Total | <span class="stat-value stat-mid">336</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Play Nice (Lv 1)
- Leer (Lv 4)
- Lick (Lv 7)
- Water Gun (Lv 10)
- Fury Swipes (Lv 13)
- Water Sport (Lv 16)
- Bite (Lv 19)
- Bubble Beam (Lv 22)
- Taunt (Lv 25)
- Fling (Lv 28)
- Extrasensory (Lv 30)
- Acrobatics (Lv 32)
- Scald (Lv 34)
- Recycle (Lv 37)
- Natural Gift (Lv 40)
- Crunch (Lv 43)
- Take Heart (Lv 46)

**Egg Moves**
- Covet
- Low Kick
- Tickle
- Nasty Plot
- Role Play
- Astonish
- Aqua Ring
- Aqua Tail
- Mud Sport
- Hydro Pump
- Disarming Voice

**Tutor Moves**
- Ice Punch
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-panpour-panel-1">
Types: Water / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Gluttony
- Cud Chew
- Torrent *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- TM56 - Scald
- HM01 - Cut
- HM03 - Surf
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Water Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="simipour" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">103</span> |
| Defense | <span class="stat-value stat-mid">63</span> |
| Sp. Atk | <span class="stat-value stat-high">103</span> |
| Sp. Def | <span class="stat-value stat-mid">63</span> |
| Speed | <span class="stat-value stat-high">98</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Play Nice (Lv 1)
- Leer (Lv 4)
- Lick (Lv 7)
- Water Gun (Lv 10)
- Fury Swipes (Lv 13)
- Water Sport (Lv 16)
- Bite (Lv 19)
- Bubble Beam (Lv 22)
- Taunt (Lv 25)
- Fling (Lv 28)
- Extrasensory (Lv 30)
- Acrobatics (Lv 32)
- Scald (Lv 34)
- Recycle (Lv 37)
- Natural Gift (Lv 40)
- Crunch (Lv 43)
- Take Heart (Lv 46)

**Egg Moves**
- Covet
- Low Kick
- Tickle
- Nasty Plot
- Role Play
- Astonish
- Aqua Ring
- Aqua Tail
- Mud Sport
- Hydro Pump
- Disarming Voice

**Tutor Moves**
- Ice Punch
- Icy Wind
- Rock Slide
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
</div>
</div>
<style>
#pokemon-tabs-panpour-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-panpour-panel-0 { display: block; }
#pokemon-tabs-panpour-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-panpour-panel-1 { display: block; }
</style>
</details>
