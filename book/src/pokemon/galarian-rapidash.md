<details class="pokemon-card-container">
<summary>Galarian Rapidash (#253)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-galarian-rapidash">
<input type="radio" name="pokemon-tabs-galarian-rapidash-group" id="pokemon-tabs-galarian-rapidash-tab-0">
<label for="pokemon-tabs-galarian-rapidash-tab-0">Galarian Ponyta</label>
<input type="radio" name="pokemon-tabs-galarian-rapidash-group" id="pokemon-tabs-galarian-rapidash-tab-1" checked>
<label for="pokemon-tabs-galarian-rapidash-tab-1">Galarian Rapidash</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-rapidash-panel-0">
Types: Psychic • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Pastel Veil
- Anticipation *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>

**Encounter Locations**
- Lastlight Road — Grass (Night) (10%)
- Riverwalk Trail (West) — Grass (Night) (20%)
- Sofos City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-ponyta" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 5)
- <a href="move-lookup.html?q=confusion">Confusion</a> (Lv 10)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 15)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 20)
- <a href="move-lookup.html?q=psybeam">Psybeam</a> (Lv 25)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 30)
- <a href="move-lookup.html?q=heal-pulse">Heal Pulse</a> (Lv 35)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 41)
- <a href="move-lookup.html?q=spirit-break">Spirit Break</a> (Lv 45)
- <a href="move-lookup.html?q=psychic">Psychic</a> (Lv 50)
- <a href="move-lookup.html?q=healing-wish">Healing Wish</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=thrash">Thrash</a>
- <a href="move-lookup.html?q=double-kick">Double Kick</a>
- <a href="move-lookup.html?q=hypnosis">Hypnosis</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=horn-drill">Horn Drill</a>
- <a href="move-lookup.html?q=morning-sun">Morning Sun</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-rapidash-panel-1">
Types: Psychic / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Pastel Veil
- Anticipation *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.25×)
- Psychic (0.5×)
- Dragon (0×)

*Weak to*
- Poison (2×)
- Ghost (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>

**Evolution Info**
Lv. 30

**Encounter Locations**
- Port Pello — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-rapidash" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">115</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=psycho-cut">Psycho Cut</a> (Lv Evo)
- <a href="move-lookup.html?q=megahorn">Megahorn</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 5)
- <a href="move-lookup.html?q=confusion">Confusion</a> (Lv 10)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 15)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 20)
- <a href="move-lookup.html?q=psybeam">Psybeam</a> (Lv 25)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 30)
- <a href="move-lookup.html?q=heal-pulse">Heal Pulse</a> (Lv 35)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 41)
- <a href="move-lookup.html?q=spirit-break">Spirit Break</a> (Lv 45)
- <a href="move-lookup.html?q=psychic">Psychic</a> (Lv 50)
- <a href="move-lookup.html?q=healing-wish">Healing Wish</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=thrash">Thrash</a>
- <a href="move-lookup.html?q=double-kick">Double Kick</a>
- <a href="move-lookup.html?q=hypnosis">Hypnosis</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=horn-drill">Horn Drill</a>
- <a href="move-lookup.html?q=morning-sun">Morning Sun</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
</div>
</div>
<style>
#pokemon-tabs-galarian-rapidash-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-rapidash-panel-0 { display: block; }
#pokemon-tabs-galarian-rapidash-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-rapidash-panel-1 { display: block; }
</style>
</details>
