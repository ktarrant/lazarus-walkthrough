<details class="pokemon-card-container">
<summary>Glalie (#187)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-glalie">
<input type="radio" name="pokemon-tabs-glalie-group" id="pokemon-tabs-glalie-tab-0">
<label for="pokemon-tabs-glalie-tab-0">Snorunt</label>
<input type="radio" name="pokemon-tabs-glalie-group" id="pokemon-tabs-glalie-tab-1" checked>
<label for="pokemon-tabs-glalie-tab-1">Glalie</label>
<input type="radio" name="pokemon-tabs-glalie-group" id="pokemon-tabs-glalie-tab-2">
<label for="pokemon-tabs-glalie-tab-2">Mega Glalie</label>
<input type="radio" name="pokemon-tabs-glalie-group" id="pokemon-tabs-glalie-tab-3">
<label for="pokemon-tabs-glalie-tab-3">Froslass</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-glalie-panel-0">
Types: Ice • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Ice Body
- Moody *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM30 - Shadow Ball
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM52 - Frost Breath
- HM05 - Flash

**Held Item**
Snowball

**Encounter Locations**
- Froslass Cavern BF1 — Grass (Day) (30%)
- Froslass Cavern BF2 — Grass (Day) (30%)
- Froslass Cavern F1 — Grass (Day) (30%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="snorunt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-low">300</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Double Team (Lv 5)
- Ice Shard (Lv 10)
- Icy Wind (Lv 14)
- Bite (Lv 19)
- Ice Fang (Lv 23)
- Headbutt (Lv 28)
- Protect (Lv 32)
- Frost Breath (Lv 37)
- Crunch (Lv 41)
- Blizzard (Lv 46)
- Hail (Lv 50)

**Egg Moves**
- Block
- Spikes
- Rollout
- Disable
- Bide
- Weather Ball
- Avalanche
- Hex
- Fake Tears
- Switcheroo

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-glalie-panel-1">
Types: Ice • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Strong Jaw
- Ice Body
- Moody *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM26 - Earthquake
- TM30 - Shadow Ball
- TM32 - Double Team
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- TM59 - Dark Pulse
- HM05 - Flash

**Evolution Info**
Lv. 32
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="glalie" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Freeze-Dry (Lv Evo)
- Deepwater Curse (Lv 1)
- Sheer Cold (Lv 1)
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Double Team (Lv 5)
- Ice Shard (Lv 10)
- Icy Wind (Lv 14)
- Bite (Lv 19)
- Ice Fang (Lv 23)
- Headbutt (Lv 28)
- Protect (Lv 32)
- Rock Slide (Lv 35)
- Frost Breath (Lv 37)
- Crunch (Lv 41)
- Avalanche (Lv 45)
- Blizzard (Lv 48)
- Hail (Lv 54)
- Sheer Cold (Lv 61)

**Egg Moves**
- Block
- Spikes
- Rollout
- Disable
- Bide
- Weather Ball
- Avalanche
- Hex
- Fake Tears
- Switcheroo

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Explosion
- Icy Wind
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-glalie-panel-2">
Types: Ice • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Refrigerate

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM26 - Earthquake
- TM30 - Shadow Ball
- TM32 - Double Team
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- TM59 - Dark Pulse
- HM05 - Flash

**Evolution Info**
Glalitite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-glalie" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">120</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-high">620</span> |

**Level-Up Moves**
- Freeze-Dry (Lv Evo)
- Deepwater Curse (Lv 1)
- Sheer Cold (Lv 1)
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Double Team (Lv 5)
- Ice Shard (Lv 10)
- Icy Wind (Lv 14)
- Bite (Lv 19)
- Ice Fang (Lv 23)
- Headbutt (Lv 28)
- Protect (Lv 32)
- Rock Slide (Lv 35)
- Frost Breath (Lv 37)
- Crunch (Lv 41)
- Avalanche (Lv 45)
- Blizzard (Lv 48)
- Hail (Lv 54)
- Sheer Cold (Lv 61)

**Egg Moves**
- Block
- Spikes
- Rollout
- Disable
- Bide
- Weather Ball
- Avalanche
- Hex
- Fake Tears
- Switcheroo

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Explosion
- Icy Wind
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-glalie-panel-3">
Types: Ice / Ghost • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Cloak
- Ice Scales
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)

*Weak to*
- Fire (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM50 - Deepwater Curse
- TM51 - Will-O-Wisp
- TM52 - Frost Breath
- TM58 - Thunder Wave
- HM05 - Flash

**Evolution Info**
Dawn Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="froslass" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">71</span> |
| Speed | <span class="stat-value stat-high">124</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Ominous Wind (Lv Evo)
- Deepwater Curse (Lv 1)
- Destiny Bond (Lv 1)
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Double Team (Lv 5)
- Ice Shard (Lv 10)
- Icy Wind (Lv 14)
- Astonish (Lv 19)
- Draining Kiss (Lv 23)
- Aurora Beam (Lv 25)
- Shadow Claw (Lv 26)
- Will-O-Wisp (Lv 28)
- Confuse Ray (Lv 32)
- Infernal Parade (Lv 35)
- Wake-Up Slap (Lv 37)
- Captivate (Lv 41)
- Shadow Ball (Lv 42)
- Ice Beam (Lv 44)
- Blizzard (Lv 48)
- Energy Ball (Lv 50)
- Hail (Lv 54)
- Destiny Bond (Lv 61)

**Egg Moves**
- Block
- Spikes
- Rollout
- Disable
- Bide
- Weather Ball
- Avalanche
- Hex
- Fake Tears
- Switcheroo

**Tutor Moves**
- Body Slam
- Dream Eater
- Endure
- Ice Punch
- Icy Wind
- Mud-Slap
- Psych Up
- Rollout
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
#pokemon-tabs-glalie-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-glalie-panel-0 { display: block; }
#pokemon-tabs-glalie-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-glalie-panel-1 { display: block; }
#pokemon-tabs-glalie-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-glalie-panel-2 { display: block; }
#pokemon-tabs-glalie-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-glalie-panel-3 { display: block; }
</style>
</details>
