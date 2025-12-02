<details class="pokemon-card-container">
<summary>Aron (#111)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-aron">
<input type="radio" name="pokemon-tabs-aron-group" id="pokemon-tabs-aron-tab-0" checked>
<label for="pokemon-tabs-aron-tab-0">Aron</label>
<input type="radio" name="pokemon-tabs-aron-group" id="pokemon-tabs-aron-tab-1">
<label for="pokemon-tabs-aron-tab-1">Lairon</label>
<input type="radio" name="pokemon-tabs-aron-group" id="pokemon-tabs-aron-tab-2">
<label for="pokemon-tabs-aron-tab-2">Aggron</label>
<input type="radio" name="pokemon-tabs-aron-group" id="pokemon-tabs-aron-tab-3">
<label for="pokemon-tabs-aron-tab-3">Mega Aggron</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-aron-panel-0">
Types: Steel / Rock • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Rock Head
- Heavy Metal *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.25×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Fighting (4×)
- Ground (4×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Hard Stone

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (8%)
- Marmaro Island — Grass (Night) (10%)
- Riverwalk Trail (West) — Grass (Day) (20%)
- Riverwalk Trail (West) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="aron" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 1)
- Mud-Slap (Lv 4)
- Headbutt (Lv 7)
- Metal Claw (Lv 10)
- Rock Tomb (Lv 13)
- Protect (Lv 16)
- Roar (Lv 19)
- Magnitude (Lv 21)
- Iron Head (Lv 22)
- Rock Slide (Lv 25)
- Take Down (Lv 28)
- Stomping Tantrum (Lv 30)
- Metal Sound (Lv 31)
- Iron Tail (Lv 34)
- Salt Cure (Lv 34)
- Iron Defense (Lv 37)
- Body Press (Lv 37)
- Double-Edge (Lv 40)
- Autotomize (Lv 43)
- Heavy Slam (Lv 46)
- Metal Burst (Lv 49)

**Egg Moves**
- Endeavor
- Body Slam
- Stomp
- Smelling Salts
- Curse
- Screech
- Iron Head
- Dragon Rush
- Head Smash
- Superpower
- Stealth Rock
- Reversal

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-aron-panel-1">
Types: Steel / Rock • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Rock Head
- Dragon's Maw *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.25×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Fighting (4×)
- Ground (4×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Hard Stone

**Evolution Info**
Lv. 28
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="lairon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">140</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">430</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 1)
- Mud-Slap (Lv 4)
- Headbutt (Lv 7)
- Metal Claw (Lv 10)
- Rock Tomb (Lv 13)
- Protect (Lv 16)
- Roar (Lv 19)
- Magnitude (Lv 21)
- Iron Head (Lv 22)
- Rock Slide (Lv 25)
- Take Down (Lv 28)
- Stomping Tantrum (Lv 30)
- Breaking Swipe (Lv 32)
- Iron Tail (Lv 35)
- Salt Cure (Lv 37)
- Iron Defense (Lv 39)
- Body Press (Lv 39)
- Scale Shot (Lv 43)
- Autotomize (Lv 47)
- Heavy Slam (Lv 51)
- Metal Burst (Lv 55)

**Egg Moves**
- Endeavor
- Body Slam
- Stomp
- Smelling Salts
- Curse
- Screech
- Iron Head
- Dragon Rush
- Head Smash
- Superpower
- Stealth Rock
- Reversal

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-aron-panel-2">
Types: Steel / Rock • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Rock Head
- Dragon's Maw *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.25×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Fighting (4×)
- Ground (4×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Hard Stone

**Evolution Info**
Lv. 42
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="aggron" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">180</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 1)
- Mud-Slap (Lv 4)
- Headbutt (Lv 7)
- Metal Claw (Lv 10)
- Rock Tomb (Lv 13)
- Protect (Lv 16)
- Roar (Lv 19)
- Magnitude (Lv 21)
- Iron Head (Lv 22)
- Rock Slide (Lv 25)
- Take Down (Lv 28)
- Stomping Tantrum (Lv 30)
- Breaking Swipe (Lv 32)
- Iron Tail (Lv 35)
- Salt Cure (Lv 37)
- Iron Defense (Lv 39)
- Body Press (Lv 39)
- Dragon Claw (Lv 43)
- Dragon Dance (Lv 47)
- Heavy Slam (Lv 51)
- Metal Burst (Lv 55)

**Egg Moves**
- Endeavor
- Body Slam
- Stomp
- Smelling Salts
- Curse
- Screech
- Iron Head
- Dragon Rush
- Head Smash
- Superpower
- Stealth Rock
- Reversal

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Fury Cutter
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-aron-panel-3">
Types: Steel • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Filter

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Hard Stone

**Evolution Info**
Aggronite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-aggron" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">140</span> |
| Defense | <span class="stat-value stat-high">230</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-high">630</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 1)
- Mud-Slap (Lv 4)
- Headbutt (Lv 7)
- Metal Claw (Lv 10)
- Rock Tomb (Lv 13)
- Protect (Lv 16)
- Roar (Lv 19)
- Magnitude (Lv 21)
- Iron Head (Lv 22)
- Rock Slide (Lv 25)
- Take Down (Lv 28)
- Stomping Tantrum (Lv 30)
- Breaking Swipe (Lv 32)
- Iron Tail (Lv 35)
- Salt Cure (Lv 37)
- Iron Defense (Lv 39)
- Body Press (Lv 39)
- Dragon Claw (Lv 43)
- Dragon Dance (Lv 47)
- Heavy Slam (Lv 51)
- Metal Burst (Lv 55)

**Egg Moves**
- Endeavor
- Body Slam
- Stomp
- Smelling Salts
- Curse
- Screech
- Iron Head
- Dragon Rush
- Head Smash
- Superpower
- Stealth Rock
- Reversal

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Fury Cutter
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
#pokemon-tabs-aron-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-aron-panel-0 { display: block; }
#pokemon-tabs-aron-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-aron-panel-1 { display: block; }
#pokemon-tabs-aron-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-aron-panel-2 { display: block; }
#pokemon-tabs-aron-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-aron-panel-3 { display: block; }
</style>
</details>
