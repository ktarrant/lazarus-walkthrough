<details class="pokemon-card-container">
<summary>Claydol (#159)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-claydol">
<input type="radio" name="pokemon-tabs-claydol-group" id="pokemon-tabs-claydol-tab-0">
<label for="pokemon-tabs-claydol-tab-0">Baltoy</label>
<input type="radio" name="pokemon-tabs-claydol-group" id="pokemon-tabs-claydol-tab-1" checked>
<label for="pokemon-tabs-claydol-tab-1">Claydol</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-claydol-panel-0">
Types: Ground / Psychic • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate
- Hustle *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.5×)
- Psychic (0.5×)
- Rock (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM23 - Hex
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM48 - Skill Swap
- TM49 - Bulldoze
- TM54 - Dazzling Gleam
- HM05 - Flash

**Held Item**
Light Clay

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (10%)
- Erinys Path (West) — Grass (Night) (20%)
- Palati City — Grass (Day) (20%)
- Palati City — Grass (Night) (20%)
- Sofos City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="baltoy" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-low">40</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">310</span> |

**Level-Up Moves**
- Harden (Lv 1)
- Confusion (Lv 1)
- Rapid Spin (Lv 4)
- Mud-Slap (Lv 7)
- Heal Block (Lv 10)
- Rock Tomb (Lv 13)
- Psybeam (Lv 16)
- Ancient Power (Lv 19)
- Cosmic Power (Lv 22)
- Power Trick (Lv 25)
- Self-Destruct (Lv 28)
- Extrasensory (Lv 31)
- Psyshield Bash (Lv 31)
- Guard Split (Lv 34)
- Power Split (Lv 34)
- Body Slam (Lv 37)
- Earth Power (Lv 40)
- Sandstorm (Lv 46)
- Diamond Storm (Lv 48)
- Imprison (Lv 52)
- Explosion (Lv 58)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Dream Eater
- Endure
- Explosion
- Mud-Slap
- Psych Up
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
<div class="pokemon-tab-panel" id="pokemon-tabs-claydol-panel-1">
Types: Ground / Psychic • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate
- Huge Power *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.5×)
- Psychic (0.5×)
- Rock (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM23 - Hex
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM48 - Skill Swap
- TM49 - Bulldoze
- TM54 - Dazzling Gleam
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash

**Held Item**
Light Clay

**Evolution Info**
Lv. 36

**Encounter Locations**
- Kaptara Island (West) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="claydol" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-high">120</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Signal Beam (Lv Evo)
- Hyper Beam (Lv 1)
- Teleport (Lv 1)
- Harden (Lv 1)
- Confusion (Lv 1)
- Rapid Spin (Lv 4)
- Mud-Slap (Lv 7)
- Heal Block (Lv 10)
- Rock Tomb (Lv 13)
- Psybeam (Lv 16)
- Ancient Power (Lv 19)
- Cosmic Power (Lv 22)
- Power Trick (Lv 25)
- Self-Destruct (Lv 28)
- Extrasensory (Lv 31)
- Psyshield Bash (Lv 31)
- Guard Split (Lv 34)
- Power Split (Lv 34)
- Body Slam (Lv 37)
- Earth Power (Lv 40)
- Sandstorm (Lv 46)
- Diamond Storm (Lv 48)
- Imprison (Lv 52)
- Explosion (Lv 58)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Dream Eater
- Endure
- Explosion
- Mud-Slap
- Psych Up
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
#pokemon-tabs-claydol-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-claydol-panel-0 { display: block; }
#pokemon-tabs-claydol-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-claydol-panel-1 { display: block; }
</style>
</details>
