<details class="pokemon-card-container">
<summary>Pawmott (#235)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-pawmott">
<input type="radio" name="pokemon-tabs-pawmott-group" id="pokemon-tabs-pawmott-tab-0">
<label for="pokemon-tabs-pawmott-tab-0">Pawmi</label>
<input type="radio" name="pokemon-tabs-pawmott-group" id="pokemon-tabs-pawmott-tab-1">
<label for="pokemon-tabs-pawmott-tab-1">Pawmo</label>
<input type="radio" name="pokemon-tabs-pawmott-group" id="pokemon-tabs-pawmott-tab-2" checked>
<label for="pokemon-tabs-pawmott-tab-2">Pawmott</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-pawmott-panel-0">
Types: Electric • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Static
- Natural Cure
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Encounter Locations**
- Asfal Hills — Grass (Day) (15%)
- Asfal Hills — Grass (Night) (15%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="pawmi" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">20</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">25</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-low">240</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 3)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 6)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 8)
- <a href="move-lookup.html?q=nuzzle">Nuzzle</a> (Lv 12)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 15)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 19)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 23)
- <a href="move-lookup.html?q=thunder-wave">Thunder Wave</a> (Lv 27)
- <a href="move-lookup.html?q=entrainment">Entrainment</a> (Lv 31)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 35)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 38)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 40)
- <a href="move-lookup.html?q=wild-charge">Wild Charge</a> (Lv 44)

**Egg Moves**
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=mach-punch">Mach Punch</a>
- <a href="move-lookup.html?q=sweet-kiss">Sweet Kiss</a>
- <a href="move-lookup.html?q=wish">Wish</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-pawmott-panel-1">
Types: Electric / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Natural Cure
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Evolution Info**
Lv. 20

**Encounter Locations**
- Palati City — Grass (Day) (10%)
- Péntepetal City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="pawmo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">76</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">74</span> |
| Total | <span class="stat-value stat-mid">350</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=arm-thrust">Arm Thrust</a> (Lv Evo)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 3)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 6)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 8)
- <a href="move-lookup.html?q=nuzzle">Nuzzle</a> (Lv 12)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 15)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 19)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 23)
- <a href="move-lookup.html?q=thunder-wave">Thunder Wave</a> (Lv 27)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 32)
- <a href="move-lookup.html?q=power-up-punch">Power-Up Punch</a> (Lv 35)
- <a href="move-lookup.html?q=entrainment">Entrainment</a> (Lv 38)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 42)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 46)
- <a href="move-lookup.html?q=wild-charge">Wild Charge</a> (Lv 52)

**Egg Moves**
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=mach-punch">Mach Punch</a>
- <a href="move-lookup.html?q=sweet-kiss">Sweet Kiss</a>
- <a href="move-lookup.html?q=wish">Wish</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-pawmott-panel-2">
Types: Electric / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Natural Cure
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm08-bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm31-brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Evolution Info**
Lv. after 1000 Steps as Follower
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="pawmott" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">74</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">96</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=revival-blessing">Revival Blessing</a> (Lv Evo)
- <a href="move-lookup.html?q=ice-punch">Ice Punch</a> (Lv 1)
- <a href="move-lookup.html?q=fire-punch">Fire Punch</a> (Lv 1)
- <a href="move-lookup.html?q=rage-fist">Rage Fist</a> (Lv 1)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=wild-charge">Wild Charge</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 3)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 6)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 8)
- <a href="move-lookup.html?q=nuzzle">Nuzzle</a> (Lv 12)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 15)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 19)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 23)
- <a href="move-lookup.html?q=arm-thrust">Arm Thrust</a> (Lv 25)
- <a href="move-lookup.html?q=thunder-wave">Thunder Wave</a> (Lv 29)
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a> (Lv 32)
- <a href="move-lookup.html?q=power-up-punch">Power-Up Punch</a> (Lv 35)
- <a href="move-lookup.html?q=entrainment">Entrainment</a> (Lv 38)
- <a href="move-lookup.html?q=close-combat">Close Combat</a> (Lv 40)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 42)
- <a href="move-lookup.html?q=double-iron-bash">Double Iron Bash</a> (Lv 45)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 47)
- <a href="move-lookup.html?q=plasma-fists">Plasma Fists</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=mach-punch">Mach Punch</a>
- <a href="move-lookup.html?q=sweet-kiss">Sweet Kiss</a>
- <a href="move-lookup.html?q=wish">Wish</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fire-punch">Fire Punch</a>
- <a href="move-lookup.html?q=ice-punch">Ice Punch</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
#pokemon-tabs-pawmott-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-pawmott-panel-0 { display: block; }
#pokemon-tabs-pawmott-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-pawmott-panel-1 { display: block; }
#pokemon-tabs-pawmott-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-pawmott-panel-2 { display: block; }
</style>
</details>
