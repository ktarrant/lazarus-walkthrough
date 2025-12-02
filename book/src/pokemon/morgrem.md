<details class="pokemon-card-container">
<summary>Morgrem (#356)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-morgrem">
<input type="radio" name="pokemon-tabs-morgrem-group" id="pokemon-tabs-morgrem-tab-0">
<label for="pokemon-tabs-morgrem-tab-0">Impidimp</label>
<input type="radio" name="pokemon-tabs-morgrem-group" id="pokemon-tabs-morgrem-tab-1" checked>
<label for="pokemon-tabs-morgrem-tab-1">Morgrem</label>
<input type="radio" name="pokemon-tabs-morgrem-group" id="pokemon-tabs-morgrem-tab-2">
<label for="pokemon-tabs-morgrem-tab-2">Grimmsnarl</label>
<input type="radio" name="pokemon-tabs-morgrem-group" id="pokemon-tabs-morgrem-tab-3">
<label for="pokemon-tabs-morgrem-tab-3">Mega Grimmsnarl</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-morgrem-panel-0">
Types: Dark / Fairy • Egg Groups: Fairy / Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Prankster
- Frisk
- Pickpocket *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0.5×)
- Dragon (0×)
- Dark (0.25×)

*Weak to*
- Poison (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM33 - Reflect
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse

**Encounter Locations**
- Wanderer's Woods (North) — Grass (Night) (20%)
- Wanderer's Woods (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="impidimp" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">30</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-low">265</span> |

**Level-Up Moves**
- Fake Out (Lv 1)
- Confide (Lv 1)
- Bite (Lv 4)
- Flatter (Lv 8)
- Fake Tears (Lv 12)
- Covet (Lv 15)
- Assurance (Lv 17)
- Swagger (Lv 20)
- Sucker Punch (Lv 24)
- Torment (Lv 28)
- Dark Pulse (Lv 33)
- Nasty Plot (Lv 36)
- Play Rough (Lv 40)
- Foul Play (Lv 44)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Endure
- Mega Kick
- Mega Punch
- Metronome
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
<div class="pokemon-tab-panel" id="pokemon-tabs-morgrem-panel-1">
Types: Dark / Fairy • Egg Groups: Fairy / Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Prankster
- Frisk
- Pickpocket *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0.5×)
- Dragon (0×)
- Dark (0.25×)

*Weak to*
- Poison (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM33 - Reflect
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse

**Evolution Info**
Lv. 24

**Encounter Locations**
- Kaptara Island (West) — Grass (Day) (10%)
- Kaptara Island (West) — Grass (Night) (10%)
- Port Pello — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="morgrem" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">380</span> |

**Level-Up Moves**
- False Surrender (Lv Evo)
- Fake Out (Lv 1)
- Confide (Lv 1)
- Bite (Lv 1)
- Flatter (Lv 1)
- Fake Tears (Lv 12)
- Covet (Lv 15)
- Assurance (Lv 17)
- Swagger (Lv 20)
- Sucker Punch (Lv 24)
- Torment (Lv 26)
- Glare (Lv 30)
- Shadow Claw (Lv 32)
- Dark Pulse (Lv 35)
- Bulk Up (Lv 40)
- Play Rough (Lv 45)
- Foul Play (Lv 50)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Endure
- Mega Kick
- Mega Punch
- Metronome
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
<div class="pokemon-tab-panel" id="pokemon-tabs-morgrem-panel-2">
Types: Dark / Fairy • Egg Groups: Fairy / Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Prankster
- Tangling Hair
- Pickpocket *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0.5×)
- Dragon (0×)
- Dark (0.25×)

*Weak to*
- Poison (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM08 - Bulk Up
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM31 - Brick Break
- TM33 - Reflect
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse

**Evolution Info**
Lv. 42

**Encounter Locations**
- Port Pello — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="grimmsnarl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Spirit Break (Lv Evo)
- False Surrender (Lv 1)
- Fake Out (Lv 1)
- Confide (Lv 1)
- Bite (Lv 1)
- Flatter (Lv 1)
- Fake Tears (Lv 12)
- Covet (Lv 15)
- Assurance (Lv 17)
- Swagger (Lv 20)
- Sucker Punch (Lv 24)
- Torment (Lv 26)
- Glare (Lv 30)
- Shadow Claw (Lv 32)
- Dark Pulse (Lv 35)
- Bulk Up (Lv 40)
- Play Rough (Lv 43)
- Power-Up Punch (Lv 46)
- Foul Play (Lv 50)
- Hammer Arm (Lv 55)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Metronome
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
<div class="pokemon-tab-panel" id="pokemon-tabs-morgrem-panel-3">
Types: Dark / Fairy • Egg Groups: Fairy / Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Unseen Fist

**Type Matchups**

*Resists / Immune to*
- Psychic (0×)
- Ghost (0.5×)
- Dragon (0×)
- Dark (0.25×)

*Weak to*
- Poison (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM08 - Bulk Up
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM31 - Brick Break
- TM33 - Reflect
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse

**Evolution Info**
Grimmsnarlite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-grimmsnarl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">160</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-high">610</span> |

**Level-Up Moves**
- Spirit Break (Lv Evo)
- False Surrender (Lv 1)
- Fake Out (Lv 1)
- Confide (Lv 1)
- Bite (Lv 1)
- Flatter (Lv 1)
- Fake Tears (Lv 12)
- Covet (Lv 15)
- Assurance (Lv 17)
- Swagger (Lv 20)
- Sucker Punch (Lv 24)
- Torment (Lv 26)
- Glare (Lv 30)
- Shadow Claw (Lv 32)
- Dark Pulse (Lv 35)
- Bulk Up (Lv 40)
- Play Rough (Lv 43)
- Power-Up Punch (Lv 46)
- Foul Play (Lv 50)
- Hammer Arm (Lv 55)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Metronome
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
#pokemon-tabs-morgrem-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-morgrem-panel-0 { display: block; }
#pokemon-tabs-morgrem-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-morgrem-panel-1 { display: block; }
#pokemon-tabs-morgrem-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-morgrem-panel-2 { display: block; }
#pokemon-tabs-morgrem-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-morgrem-panel-3 { display: block; }
</style>
</details>
