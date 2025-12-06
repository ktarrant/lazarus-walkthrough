<details class="pokemon-card-container">
<summary>Electrode (#364)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-electrode">
<input type="radio" name="pokemon-tabs-electrode-group" id="pokemon-tabs-electrode-tab-0">
<label for="pokemon-tabs-electrode-tab-0">Voltorb</label>
<input type="radio" name="pokemon-tabs-electrode-group" id="pokemon-tabs-electrode-tab-1" checked>
<label for="pokemon-tabs-electrode-tab-1">Electrode</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-electrode-panel-0">
Types: Electric • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Soundproof
- Static
- Aftermath *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=flash">HM05 - Flash</a>

**Encounter Locations**
- Jusmail Town — Grass (Night) (10%)
- Sea of Vulcai — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="voltorb" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=sonic-boom">Sonic Boom</a> (Lv 4)
- <a href="move-lookup.html?q=eerie-impulse">Eerie Impulse</a> (Lv 6)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 9)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 11)
- <a href="move-lookup.html?q=screech">Screech</a> (Lv 13)
- <a href="move-lookup.html?q=charge-beam">Charge Beam</a> (Lv 16)
- <a href="move-lookup.html?q=swift">Swift</a> (Lv 20)
- <a href="move-lookup.html?q=electro-ball">Electro Ball</a> (Lv 22)
- <a href="move-lookup.html?q=self-destruct">Self-Destruct</a> (Lv 26)
- <a href="move-lookup.html?q=light-screen">Light Screen</a> (Lv 29)
- <a href="move-lookup.html?q=magnet-rise">Magnet Rise</a> (Lv 34)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 37)
- <a href="move-lookup.html?q=explosion">Explosion</a> (Lv 41)
- <a href="move-lookup.html?q=gyro-ball">Gyro Ball</a> (Lv 46)
- <a href="move-lookup.html?q=mirror-coat">Mirror Coat</a> (Lv 48)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-electrode-panel-1">
Types: Electric • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Soundproof
- Static
- Aftermath *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=flash">HM05 - Flash</a>

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="electrode" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">150</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=magnetic-flux">Magnetic Flux</a> (Lv 1)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=sonic-boom">Sonic Boom</a> (Lv 4)
- <a href="move-lookup.html?q=eerie-impulse">Eerie Impulse</a> (Lv 6)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 9)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 11)
- <a href="move-lookup.html?q=screech">Screech</a> (Lv 13)
- <a href="move-lookup.html?q=charge-beam">Charge Beam</a> (Lv 16)
- <a href="move-lookup.html?q=swift">Swift</a> (Lv 20)
- <a href="move-lookup.html?q=electro-ball">Electro Ball</a> (Lv 22)
- <a href="move-lookup.html?q=self-destruct">Self-Destruct</a> (Lv 26)
- <a href="move-lookup.html?q=light-screen">Light Screen</a> (Lv 29)
- <a href="move-lookup.html?q=magnet-rise">Magnet Rise</a> (Lv 36)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 41)
- <a href="move-lookup.html?q=explosion">Explosion</a> (Lv 47)
- <a href="move-lookup.html?q=gyro-ball">Gyro Ball</a> (Lv 54)
- <a href="move-lookup.html?q=mirror-coat">Mirror Coat</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
#pokemon-tabs-electrode-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-electrode-panel-0 { display: block; }
#pokemon-tabs-electrode-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-electrode-panel-1 { display: block; }
</style>
</details>
