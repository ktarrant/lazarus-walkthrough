<details class="pokemon-card-container">
<summary>Kilowattrel (#243)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-kilowattrel">
<input type="radio" name="pokemon-tabs-kilowattrel-group" id="pokemon-tabs-kilowattrel-tab-0">
<label for="pokemon-tabs-kilowattrel-tab-0">Wattrel</label>
<input type="radio" name="pokemon-tabs-kilowattrel-group" id="pokemon-tabs-kilowattrel-tab-1" checked>
<label for="pokemon-tabs-kilowattrel-tab-1">Kilowattrel</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-kilowattrel-panel-0">
Types: Electric / Flying • Egg Groups: Water 1 / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Wind Power
- Volt Absorb
- Competitive *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Ice (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=fly">HM02 - Fly</a>

**Encounter Locations**
- Pythios Cemetery — Grass (Day) (20%)
- Sea of Asteri (East) — Surfing (30%)
- Sea of Asteri (West) — Grass (Day) (15%)
- Sea of Asteri (West) — Grass (Night) (15%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="wattrel" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">40</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-low">280</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=peck">Peck</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 4)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 7)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 11)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 15)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 19)
- <a href="move-lookup.html?q=double-team">Double Team</a> (Lv 21)
- <a href="move-lookup.html?q=roost">Roost</a> (Lv 23)
- <a href="move-lookup.html?q=dual-wingbeat">Dual Wingbeat</a> (Lv 28)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 32)
- <a href="move-lookup.html?q=volt-switch">Volt Switch</a> (Lv 35)
- <a href="move-lookup.html?q=air-slash">Air Slash</a> (Lv 39)
- <a href="move-lookup.html?q=flying-press">Flying Press</a> (Lv 43)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 45)
- <a href="move-lookup.html?q=hurricane">Hurricane</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=endeavor">Endeavor</a>
- <a href="move-lookup.html?q=feather-dance">Feather Dance</a>
- <a href="move-lookup.html?q=spit-up">Spit Up</a>
- <a href="move-lookup.html?q=stockpile">Stockpile</a>
- <a href="move-lookup.html?q=swallow">Swallow</a>
- <a href="move-lookup.html?q=weather-ball">Weather Ball</a>

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
<div class="pokemon-tab-panel" id="pokemon-tabs-kilowattrel-panel-1">
Types: Electric / Flying • Egg Groups: Water 1 / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Wind Power
- Volt Absorb
- Competitive *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Ice (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=fly">HM02 - Fly</a>

**Evolution Info**
Lv. 25

**Encounter Locations**
- Sea of Vulcai — Grass (Day) (5%)
- Sea of Vulcai — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="kilowattrel" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">105</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">125</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=electro-ball">Electro Ball</a> (Lv Evo)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=peck">Peck</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 4)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 7)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 11)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 15)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 19)
- <a href="move-lookup.html?q=double-team">Double Team</a> (Lv 21)
- <a href="move-lookup.html?q=roost">Roost</a> (Lv 24)
- <a href="move-lookup.html?q=dual-wingbeat">Dual Wingbeat</a> (Lv 28)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 32)
- <a href="move-lookup.html?q=volt-switch">Volt Switch</a> (Lv 35)
- <a href="move-lookup.html?q=air-slash">Air Slash</a> (Lv 39)
- <a href="move-lookup.html?q=flying-press">Flying Press</a> (Lv 43)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 45)
- <a href="move-lookup.html?q=hurricane">Hurricane</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=endeavor">Endeavor</a>
- <a href="move-lookup.html?q=feather-dance">Feather Dance</a>
- <a href="move-lookup.html?q=spit-up">Spit Up</a>
- <a href="move-lookup.html?q=stockpile">Stockpile</a>
- <a href="move-lookup.html?q=swallow">Swallow</a>
- <a href="move-lookup.html?q=weather-ball">Weather Ball</a>

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
</div>
</div>
<style>
#pokemon-tabs-kilowattrel-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-kilowattrel-panel-0 { display: block; }
#pokemon-tabs-kilowattrel-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-kilowattrel-panel-1 { display: block; }
</style>
</details>
