<details class="pokemon-card-container">
<summary>Yamper (#231)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-yamper">
<input type="radio" name="pokemon-tabs-yamper-group" id="pokemon-tabs-yamper-tab-0" checked>
<label for="pokemon-tabs-yamper-tab-0">Yamper</label>
<input type="radio" name="pokemon-tabs-yamper-group" id="pokemon-tabs-yamper-tab-1">
<label for="pokemon-tabs-yamper-tab-1">Boltund</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-yamper-panel-0">
Types: Electric • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Ball Fetch
- Competitive
- Rattled *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>

**Encounter Locations**
- Asfal Hills — Grass (Day) (10%)
- Asfal Hills — Grass (Night) (10%)
- Bronze Fields (South) — Grass (Night) (5%)
- Riverwalk Trail (West) — Grass (Day) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="yamper" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">59</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">26</span> |
| Total | <span class="stat-value stat-low">270</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=nuzzle">Nuzzle</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 14)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 18)
- <a href="move-lookup.html?q=charm">Charm</a> (Lv 21)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 24)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 27)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 30)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 32)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 35)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 38)
- <a href="move-lookup.html?q=wild-charge">Wild Charge</a> (Lv 40)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 43)
- <a href="move-lookup.html?q=volt-switch">Volt Switch</a> (Lv 46)
- <a href="move-lookup.html?q=mirror-shot">Mirror Shot</a> (Lv 50)
- <a href="move-lookup.html?q=electric-terrain">Electric Terrain</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a>
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a>
- <a href="move-lookup.html?q=discharge">Discharge</a>
- <a href="move-lookup.html?q=howl">Howl</a>

**Tutor Moves**
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
<div class="pokemon-tab-panel" id="pokemon-tabs-yamper-panel-1">
Types: Electric • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Strong Jaw
- Competitive
- Tough Claws *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>

**Evolution Info**
Lv. 25

**Encounter Locations**
- Areios Hideout — Grass (Day) (10%)
- Areios Hideout — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="boltund" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">74</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">121</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=thunder-fang">Thunder Fang</a> (Lv Evo)
- <a href="move-lookup.html?q=electrify">Electrify</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=nuzzle">Nuzzle</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 14)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 18)
- <a href="move-lookup.html?q=charm">Charm</a> (Lv 21)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 24)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 27)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 30)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 32)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 35)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 38)
- <a href="move-lookup.html?q=wild-charge">Wild Charge</a> (Lv 40)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 43)
- <a href="move-lookup.html?q=volt-switch">Volt Switch</a> (Lv 46)
- <a href="move-lookup.html?q=mirror-shot">Mirror Shot</a> (Lv 50)
- <a href="move-lookup.html?q=electric-terrain">Electric Terrain</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a>
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a>
- <a href="move-lookup.html?q=discharge">Discharge</a>
- <a href="move-lookup.html?q=howl">Howl</a>

**Tutor Moves**
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
</div>
</div>
<style>
#pokemon-tabs-yamper-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-yamper-panel-0 { display: block; }
#pokemon-tabs-yamper-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-yamper-panel-1 { display: block; }
</style>
</details>
