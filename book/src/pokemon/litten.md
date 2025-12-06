<details class="pokemon-card-container">
<summary>Litten (#004)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-litten">
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-0" checked>
<label for="pokemon-tabs-litten-tab-0">Litten</label>
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-1">
<label for="pokemon-tabs-litten-tab-1">Torracat</label>
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-2">
<label for="pokemon-tabs-litten-tab-2">Incineroar</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>

**Encounter Locations**
- Sea of Vulcai — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="litten" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">320</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 4)
- <a href="move-lookup.html?q=lick">Lick</a> (Lv 8)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 11)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 14)
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 16)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 18)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 22)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 25)
- <a href="move-lookup.html?q=fury-swipes">Fury Swipes</a> (Lv 29)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 32)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 35)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 39)
- <a href="move-lookup.html?q=flare-blitz">Flare Blitz</a> (Lv 43)
- <a href="move-lookup.html?q=outrage">Outrage</a> (Lv 46)

**Egg Moves**
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a>
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=crunch">Crunch</a>
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=revenge">Revenge</a>
- <a href="move-lookup.html?q=heat-wave">Heat Wave</a>
- <a href="move-lookup.html?q=power-trip">Power Trip</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>

**Evolution Info**
Lv. 17
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="torracat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 4)
- <a href="move-lookup.html?q=lick">Lick</a> (Lv 8)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 11)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 14)
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 16)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 18)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 22)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 25)
- <a href="move-lookup.html?q=fury-swipes">Fury Swipes</a> (Lv 29)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 32)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 35)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 39)
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a> (Lv 42)
- <a href="move-lookup.html?q=flare-blitz">Flare Blitz</a> (Lv 46)
- <a href="move-lookup.html?q=outrage">Outrage</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a>
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=crunch">Crunch</a>
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=revenge">Revenge</a>
- <a href="move-lookup.html?q=heat-wave">Heat Wave</a>
- <a href="move-lookup.html?q=power-trip">Power Trip</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-2">
Types: Fire / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Water (2×)
- Fighting (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

**Evolution Info**
Lv. 34
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="incineroar" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=darkest-lariat">Darkest Lariat</a> (Lv Evo)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 1)
- <a href="move-lookup.html?q=throat-chop">Throat Chop</a> (Lv 1)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 4)
- <a href="move-lookup.html?q=lick">Lick</a> (Lv 8)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 11)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 14)
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 16)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 18)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 22)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 25)
- <a href="move-lookup.html?q=fury-swipes">Fury Swipes</a> (Lv 29)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 32)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 35)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 39)
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a> (Lv 42)
- <a href="move-lookup.html?q=flare-blitz">Flare Blitz</a> (Lv 46)
- <a href="move-lookup.html?q=outrage">Outrage</a> (Lv 50)
- <a href="move-lookup.html?q=cross-chop">Cross Chop</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a>
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=crunch">Crunch</a>
- <a href="move-lookup.html?q=fake-out">Fake Out</a>
- <a href="move-lookup.html?q=revenge">Revenge</a>
- <a href="move-lookup.html?q=heat-wave">Heat Wave</a>
- <a href="move-lookup.html?q=power-trip">Power Trip</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fire-punch">Fire Punch</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
#pokemon-tabs-litten-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-0 { display: block; }
#pokemon-tabs-litten-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-1 { display: block; }
#pokemon-tabs-litten-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-2 { display: block; }
</style>
</details>
