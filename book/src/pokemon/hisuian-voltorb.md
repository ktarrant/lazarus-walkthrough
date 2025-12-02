<details class="pokemon-card-container">
<summary>Hisuian Voltorb (#363)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-hisuian-voltorb">
<input type="radio" name="pokemon-tabs-hisuian-voltorb-group" id="pokemon-tabs-hisuian-voltorb-tab-0" checked>
<label for="pokemon-tabs-hisuian-voltorb-tab-0">Hisuian Voltorb</label>
<input type="radio" name="pokemon-tabs-hisuian-voltorb-group" id="pokemon-tabs-hisuian-voltorb-tab-1">
<label for="pokemon-tabs-hisuian-voltorb-tab-1">Hisuian Electrode</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-voltorb-panel-0">
Types: Electric / Grass • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Seed Sower
- Static
- Aftermath *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.25×)
- Grass (0.5×)
- Steel (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Encounter Locations**
- Jusmail Town — Grass (Night) (10%)
- Sea of Vulcai — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-voltorb" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 5)
- <a href="move-lookup.html?q=thunder-wave">Thunder Wave</a> (Lv 8)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 11)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 15)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 19)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 24)
- <a href="move-lookup.html?q=grassy-glide">Grassy Glide</a> (Lv 27)
- <a href="move-lookup.html?q=charge-beam">Charge Beam</a> (Lv 31)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 36)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 41)
- <a href="move-lookup.html?q=self-destruct">Self-Destruct</a> (Lv 47)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-voltorb-panel-1">
Types: Electric / Grass • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Seed Sower
- Static
- Aftermath *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.25×)
- Grass (0.5×)
- Steel (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Evolution Info**
Leaf Stone

**Encounter Locations**
- Nyx Trails — Grass (Day) (2%)
- Nyx Trails — Grass (Night) (2%)
- Pollen Road — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-electrode" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">140</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=chloroblast">Chloroblast</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 5)
- <a href="move-lookup.html?q=thunder-wave">Thunder Wave</a> (Lv 8)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 11)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 15)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 19)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 24)
- <a href="move-lookup.html?q=grassy-glide">Grassy Glide</a> (Lv 27)
- <a href="move-lookup.html?q=charge-beam">Charge Beam</a> (Lv 31)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 36)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 41)
- <a href="move-lookup.html?q=self-destruct">Self-Destruct</a> (Lv 47)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
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
#pokemon-tabs-hisuian-voltorb-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-voltorb-panel-0 { display: block; }
#pokemon-tabs-hisuian-voltorb-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-voltorb-panel-1 { display: block; }
</style>
</details>
