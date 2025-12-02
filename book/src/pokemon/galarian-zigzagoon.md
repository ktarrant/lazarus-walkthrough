<details class="pokemon-card-container">
<summary>Galarian Zigzagoon (#348)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-galarian-zigzagoon">
<input type="radio" name="pokemon-tabs-galarian-zigzagoon-group" id="pokemon-tabs-galarian-zigzagoon-tab-0" checked>
<label for="pokemon-tabs-galarian-zigzagoon-tab-0">Galarian Zigzagoon</label>
<input type="radio" name="pokemon-tabs-galarian-zigzagoon-group" id="pokemon-tabs-galarian-zigzagoon-tab-1">
<label for="pokemon-tabs-galarian-zigzagoon-tab-1">Galarian Linoone</label>
<input type="radio" name="pokemon-tabs-galarian-zigzagoon-group" id="pokemon-tabs-galarian-zigzagoon-tab-2">
<label for="pokemon-tabs-galarian-zigzagoon-tab-2">Obstagoon</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-zigzagoon-panel-0">
Types: Dark / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pickup
- Gluttony
- Quick Feet *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (4×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM30 - Shadow Ball
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM55 - Snarl
- TM58 - Thunder Wave
- HM03 - Surf

**Encounter Locations**
- Erinys Path (West) — Grass (Night) (10%)
- Riverwalk Trail (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-zigzagoon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">38</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">41</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">41</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-low">240</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 1)
- Sand Attack (Lv 3)
- Lick (Lv 6)
- Snarl (Lv 9)
- Headbutt (Lv 12)
- Baby-Doll Eyes (Lv 15)
- Pin Missile (Lv 18)
- Rest (Lv 21)
- Take Down (Lv 24)
- Scary Face (Lv 27)
- Counter (Lv 30)
- Taunt (Lv 33)
- Double-Edge (Lv 36)

**Egg Moves**
- Parting Shot
- Quick Guard
- Knock Off

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Icy Wind
- Sleep Talk
- Snore
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
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-zigzagoon-panel-1">
Types: Dark / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pickup
- Gluttony
- Quick Feet *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (4×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM30 - Shadow Ball
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM55 - Snarl
- TM58 - Thunder Wave
- HM03 - Surf

**Evolution Info**
Lv. 20

**Encounter Locations**
- Port Pello — Grass (Night) (10%)
- Péntepetal City — Grass (Night) (10%)
- Sea of Asteri (West) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-linoone" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">78</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-mid">61</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-mid">61</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- Night Slash (Lv Evo)
- Switcheroo (Lv 1)
- Pin Missile (Lv 1)
- Baby-Doll Eyes (Lv 1)
- Tackle (Lv 1)
- Leer (Lv 1)
- Sand Attack (Lv 1)
- Lick (Lv 1)
- Snarl (Lv 9)
- Headbutt (Lv 12)
- Hone Claws (Lv 15)
- Fury Swipes (Lv 18)
- Rest (Lv 23)
- Take Down (Lv 28)
- Scary Face (Lv 33)
- Counter (Lv 38)
- Taunt (Lv 43)
- Double-Edge (Lv 48)

**Egg Moves**
- Parting Shot
- Quick Guard
- Knock Off

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Icy Wind
- Sleep Talk
- Snore
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
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-zigzagoon-panel-2">
Types: Dark / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Reckless
- Guts
- Defiant *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (4×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM08 - Bulk Up
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM55 - Snarl
- TM58 - Thunder Wave
- HM03 - Surf

**Evolution Info**
Lv. 35, Night

**Encounter Locations**
- Areios Hideout — Grass (Day) (10%)
- Areios Hideout — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="obstagoon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">93</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-high">101</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-mid">81</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Obstruct (Lv Evo)
- Cross Chop (Lv 1)
- Submission (Lv 1)
- Night Slash (Lv 1)
- Switcheroo (Lv 1)
- Pin Missile (Lv 1)
- Baby-Doll Eyes (Lv 1)
- Tackle (Lv 1)
- Leer (Lv 1)
- Sand Attack (Lv 1)
- Lick (Lv 1)
- Snarl (Lv 9)
- Headbutt (Lv 12)
- Hone Claws (Lv 15)
- Fury Swipes (Lv 18)
- Rest (Lv 23)
- Take Down (Lv 28)
- Scary Face (Lv 35)
- Counter (Lv 42)
- Taunt (Lv 49)
- Double-Edge (Lv 56)

**Egg Moves**
- Parting Shot
- Quick Guard
- Knock Off

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Fire Punch
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Sleep Talk
- Snore
- Swift
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
#pokemon-tabs-galarian-zigzagoon-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-zigzagoon-panel-0 { display: block; }
#pokemon-tabs-galarian-zigzagoon-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-zigzagoon-panel-1 { display: block; }
#pokemon-tabs-galarian-zigzagoon-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-zigzagoon-panel-2 { display: block; }
</style>
</details>
