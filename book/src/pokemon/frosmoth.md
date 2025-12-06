<details class="pokemon-card-container">
<summary>Frosmoth (#056)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-frosmoth">
<input type="radio" name="pokemon-tabs-frosmoth-group" id="pokemon-tabs-frosmoth-tab-0">
<label for="pokemon-tabs-frosmoth-tab-0">Snom</label>
<input type="radio" name="pokemon-tabs-frosmoth-group" id="pokemon-tabs-frosmoth-tab-1" checked>
<label for="pokemon-tabs-frosmoth-tab-1">Frosmoth</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-frosmoth-panel-0">
Types: Ice / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shield Dust
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ice (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Flying (2×)
- Rock (4×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>

**Held Item**
Snowball

**Encounter Locations**
- Froslass Cavern BF2 — Grass (Day) (5%)
- Pythios Cemetery — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="snom" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">30</span> |
| Attack | <span class="stat-value stat-low">25</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-low">185</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=powder-snow">Powder Snow</a> (Lv 1)
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a> (Lv 1)
- <a href="move-lookup.html?q=sticky-web">Sticky Web</a> (Lv 15)

**Egg Moves**
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a>
- <a href="move-lookup.html?q=mirror-coat">Mirror Coat</a>
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-frosmoth-panel-1">
Types: Ice / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shield Dust
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ice (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Flying (2×)
- Rock (4×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>

**Evolution Info**
Ice Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="frosmoth" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a> (Lv Evo)
- <a href="move-lookup.html?q=powder-snow">Powder Snow</a> (Lv 1)
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a> (Lv 1)
- <a href="move-lookup.html?q=helping-hand">Helping Hand</a> (Lv 1)
- <a href="move-lookup.html?q=attract">Attract</a> (Lv 1)
- <a href="move-lookup.html?q=sticky-web">Sticky Web</a> (Lv 1)
- <a href="move-lookup.html?q=stun-spore">Stun Spore</a> (Lv 4)
- <a href="move-lookup.html?q=infestation">Infestation</a> (Lv 8)
- <a href="move-lookup.html?q=mist">Mist</a> (Lv 12)
- <a href="move-lookup.html?q=defog">Defog</a> (Lv 16)
- <a href="move-lookup.html?q=mega-drain">Mega Drain</a> (Lv 20)
- <a href="move-lookup.html?q=feather-dance">Feather Dance</a> (Lv 22)
- <a href="move-lookup.html?q=aurora-beam">Aurora Beam</a> (Lv 24)
- <a href="move-lookup.html?q=hail">Hail</a> (Lv 28)
- <a href="move-lookup.html?q=freeze-dry">Freeze-Dry</a> (Lv 30)
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a> (Lv 32)
- <a href="move-lookup.html?q=aurora-veil">Aurora Veil</a> (Lv 35)
- <a href="move-lookup.html?q=quiver-dance">Quiver Dance</a> (Lv 38)
- <a href="move-lookup.html?q=blizzard">Blizzard</a> (Lv 40)
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a> (Lv 42)
- <a href="move-lookup.html?q=tailwind">Tailwind</a> (Lv 45)
- <a href="move-lookup.html?q=wide-guard">Wide Guard</a> (Lv 48)
- <a href="move-lookup.html?q=quiver-dance">Quiver Dance</a> (Lv 52)

**Egg Moves**
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a>
- <a href="move-lookup.html?q=mirror-coat">Mirror Coat</a>
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
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
#pokemon-tabs-frosmoth-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-frosmoth-panel-0 { display: block; }
#pokemon-tabs-frosmoth-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-frosmoth-panel-1 { display: block; }
</style>
</details>
