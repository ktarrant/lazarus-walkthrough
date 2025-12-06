<details class="pokemon-card-container">
<summary>Mega Grimmsnarl (#357)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-mega-grimmsnarl">
<input type="radio" name="pokemon-tabs-mega-grimmsnarl-group" id="pokemon-tabs-mega-grimmsnarl-tab-0">
<label for="pokemon-tabs-mega-grimmsnarl-tab-0">Impidimp</label>
<input type="radio" name="pokemon-tabs-mega-grimmsnarl-group" id="pokemon-tabs-mega-grimmsnarl-tab-1">
<label for="pokemon-tabs-mega-grimmsnarl-tab-1">Morgrem</label>
<input type="radio" name="pokemon-tabs-mega-grimmsnarl-group" id="pokemon-tabs-mega-grimmsnarl-tab-2">
<label for="pokemon-tabs-mega-grimmsnarl-tab-2">Grimmsnarl</label>
<input type="radio" name="pokemon-tabs-mega-grimmsnarl-group" id="pokemon-tabs-mega-grimmsnarl-tab-3" checked>
<label for="pokemon-tabs-mega-grimmsnarl-tab-3">Mega Grimmsnarl</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-grimmsnarl-panel-0">
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
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

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
- <a href="move-lookup.html?q=fake-out">Fake Out</a> (Lv 1)
- <a href="move-lookup.html?q=confide">Confide</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 4)
- <a href="move-lookup.html?q=flatter">Flatter</a> (Lv 8)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 12)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 15)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 17)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 20)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 24)
- <a href="move-lookup.html?q=torment">Torment</a> (Lv 28)
- <a href="move-lookup.html?q=dark-pulse">Dark Pulse</a> (Lv 33)
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a> (Lv 36)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 40)
- <a href="move-lookup.html?q=foul-play">Foul Play</a> (Lv 44)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-grimmsnarl-panel-1">
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
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

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
- <a href="move-lookup.html?q=false-surrender">False Surrender</a> (Lv Evo)
- <a href="move-lookup.html?q=fake-out">Fake Out</a> (Lv 1)
- <a href="move-lookup.html?q=confide">Confide</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=flatter">Flatter</a> (Lv 1)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 12)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 15)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 17)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 20)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 24)
- <a href="move-lookup.html?q=torment">Torment</a> (Lv 26)
- <a href="move-lookup.html?q=glare">Glare</a> (Lv 30)
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv 32)
- <a href="move-lookup.html?q=dark-pulse">Dark Pulse</a> (Lv 35)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 40)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 45)
- <a href="move-lookup.html?q=foul-play">Foul Play</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-grimmsnarl-panel-2">
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
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=power-up-punch">TM53 - Power-Up Punch</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

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
- <a href="move-lookup.html?q=spirit-break">Spirit Break</a> (Lv Evo)
- <a href="move-lookup.html?q=false-surrender">False Surrender</a> (Lv 1)
- <a href="move-lookup.html?q=fake-out">Fake Out</a> (Lv 1)
- <a href="move-lookup.html?q=confide">Confide</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=flatter">Flatter</a> (Lv 1)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 12)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 15)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 17)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 20)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 24)
- <a href="move-lookup.html?q=torment">Torment</a> (Lv 26)
- <a href="move-lookup.html?q=glare">Glare</a> (Lv 30)
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv 32)
- <a href="move-lookup.html?q=dark-pulse">Dark Pulse</a> (Lv 35)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 40)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 43)
- <a href="move-lookup.html?q=power-up-punch">Power-Up Punch</a> (Lv 46)
- <a href="move-lookup.html?q=foul-play">Foul Play</a> (Lv 50)
- <a href="move-lookup.html?q=hammer-arm">Hammer Arm</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fire-punch">Fire Punch</a>
- <a href="move-lookup.html?q=ice-punch">Ice Punch</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-grimmsnarl-panel-3">
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
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=power-up-punch">TM53 - Power-Up Punch</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

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
- <a href="move-lookup.html?q=spirit-break">Spirit Break</a> (Lv Evo)
- <a href="move-lookup.html?q=false-surrender">False Surrender</a> (Lv 1)
- <a href="move-lookup.html?q=fake-out">Fake Out</a> (Lv 1)
- <a href="move-lookup.html?q=confide">Confide</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=flatter">Flatter</a> (Lv 1)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 12)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 15)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 17)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 20)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 24)
- <a href="move-lookup.html?q=torment">Torment</a> (Lv 26)
- <a href="move-lookup.html?q=glare">Glare</a> (Lv 30)
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv 32)
- <a href="move-lookup.html?q=dark-pulse">Dark Pulse</a> (Lv 35)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 40)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 43)
- <a href="move-lookup.html?q=power-up-punch">Power-Up Punch</a> (Lv 46)
- <a href="move-lookup.html?q=foul-play">Foul Play</a> (Lv 50)
- <a href="move-lookup.html?q=hammer-arm">Hammer Arm</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fire-punch">Fire Punch</a>
- <a href="move-lookup.html?q=ice-punch">Ice Punch</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
#pokemon-tabs-mega-grimmsnarl-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-grimmsnarl-panel-0 { display: block; }
#pokemon-tabs-mega-grimmsnarl-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-grimmsnarl-panel-1 { display: block; }
#pokemon-tabs-mega-grimmsnarl-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-grimmsnarl-panel-2 { display: block; }
#pokemon-tabs-mega-grimmsnarl-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-grimmsnarl-panel-3 { display: block; }
</style>
</details>
