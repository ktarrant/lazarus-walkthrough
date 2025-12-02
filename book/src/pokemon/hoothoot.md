<details class="pokemon-card-container">
<summary>Hoothoot (#141)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-hoothoot">
<input type="radio" name="pokemon-tabs-hoothoot-group" id="pokemon-tabs-hoothoot-tab-0" checked>
<label for="pokemon-tabs-hoothoot-tab-0">Hoothoot</label>
<input type="radio" name="pokemon-tabs-hoothoot-group" id="pokemon-tabs-hoothoot-tab-1">
<label for="pokemon-tabs-hoothoot-tab-1">Noctowl</label>
<input type="radio" name="pokemon-tabs-hoothoot-group" id="pokemon-tabs-hoothoot-tab-2">
<label for="pokemon-tabs-hoothoot-tab-2">Mega Noctowl</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-hoothoot-panel-0">
Types: Normal / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Analytic
- Keen Eye
- Tinted Lens *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ground (0×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM48 - Skill Swap
- TM50 - Deepwater Curse
- TM51 - Will-O-Wisp
- TM57 - Roost
- HM02 - Fly
- HM05 - Flash

**Encounter Locations**
- Acrisia City — Grass (Night) (20%)
- Bronze Fields (North) — Grass (Night) (20%)
- Pythios Town — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hoothoot" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">30</span> |
| Sp. Atk | <span class="stat-value stat-low">41</span> |
| Sp. Def | <span class="stat-value stat-mid">56</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-low">267</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Growl (Lv 1)
- Foresight (Lv 1)
- Hypnosis (Lv 4)
- Peck (Lv 7)
- Confusion (Lv 10)
- Echoed Voice (Lv 13)
- Night Shade (Lv 16)
- Psycho Shift (Lv 19)
- Extrasensory (Lv 22)
- Relic Song (Lv 25)
- Reflect (Lv 28)
- Air Slash (Lv 31)
- Uproar (Lv 34)
- Roost (Lv 37)
- Moonblast (Lv 40)
- Synchronoise (Lv 43)
- Dream Eater (Lv 46)

**Egg Moves**
- Mirror Move
- Supersonic
- Feint Attack
- Wing Attack
- Whirlwind
- Sky Attack
- Feather Dance
- Agility
- Night Shade
- Defog
- Mean Look
- Hurricane

**Tutor Moves**
- Double-Edge
- Dream Eater
- Endure
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hoothoot-panel-1">
Types: Ghost / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shadow Tag
- Keen Eye
- Tinted Lens *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Ground (0×)
- Bug (0.25×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM48 - Skill Swap
- TM50 - Deepwater Curse
- TM51 - Will-O-Wisp
- TM57 - Roost
- HM02 - Fly
- HM05 - Flash

**Evolution Info**
Lv. 20

**Encounter Locations**
- Sea of Vulcai — Grass (Night) (10%)
- Sofos City — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="noctowl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-mid">86</span> |
| Sp. Def | <span class="stat-value stat-high">101</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">457</span> |

**Level-Up Moves**
- Hex (Lv Evo)
- Dream Eater (Lv 1)
- Sky Attack (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Foresight (Lv 1)
- Hypnosis (Lv 4)
- Peck (Lv 7)
- Confusion (Lv 10)
- Echoed Voice (Lv 13)
- Night Shade (Lv 16)
- Psycho Shift (Lv 19)
- Psybeam (Lv 23)
- Relic Song (Lv 27)
- Extrasensory (Lv 30)
- Reflect (Lv 32)
- Air Slash (Lv 35)
- Bitter Malice (Lv 38)
- Uproar (Lv 41)
- Roost (Lv 43)
- Moonblast (Lv 47)
- Synchronoise (Lv 51)
- Dream Eater (Lv 55)

**Egg Moves**
- Mirror Move
- Supersonic
- Feint Attack
- Wing Attack
- Whirlwind
- Sky Attack
- Feather Dance
- Agility
- Night Shade
- Defog
- Mean Look
- Hurricane

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hoothoot-panel-2">
Types: Ghost / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Analytic

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Ground (0×)
- Bug (0.25×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM48 - Skill Swap
- TM50 - Deepwater Curse
- TM51 - Will-O-Wisp
- TM57 - Roost
- HM02 - Fly
- HM05 - Flash

**Evolution Info**
Noctowlite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-noctowl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-high">121</span> |
| Sp. Def | <span class="stat-value stat-high">141</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-high">557</span> |

**Level-Up Moves**
- Hex (Lv Evo)
- Dream Eater (Lv 1)
- Sky Attack (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Foresight (Lv 1)
- Hypnosis (Lv 4)
- Peck (Lv 7)
- Confusion (Lv 10)
- Echoed Voice (Lv 13)
- Night Shade (Lv 16)
- Psycho Shift (Lv 19)
- Psybeam (Lv 23)
- Relic Song (Lv 27)
- Extrasensory (Lv 30)
- Reflect (Lv 32)
- Air Slash (Lv 35)
- Bitter Malice (Lv 38)
- Uproar (Lv 41)
- Roost (Lv 43)
- Moonblast (Lv 47)
- Synchronoise (Lv 51)
- Dream Eater (Lv 55)

**Egg Moves**
- Mirror Move
- Supersonic
- Feint Attack
- Wing Attack
- Whirlwind
- Sky Attack
- Feather Dance
- Agility
- Night Shade
- Defog
- Mean Look
- Hurricane

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swift
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
#pokemon-tabs-hoothoot-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-hoothoot-panel-0 { display: block; }
#pokemon-tabs-hoothoot-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-hoothoot-panel-1 { display: block; }
#pokemon-tabs-hoothoot-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-hoothoot-panel-2 { display: block; }
</style>
</details>
