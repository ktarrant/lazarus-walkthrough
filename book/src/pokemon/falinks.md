<details class="pokemon-card-container">
<summary>Falinks (#320)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-falinks">
<input type="radio" name="pokemon-tabs-falinks-group" id="pokemon-tabs-falinks-tab-0" checked>
<label for="pokemon-tabs-falinks-tab-0">Falinks</label>
<input type="radio" name="pokemon-tabs-falinks-group" id="pokemon-tabs-falinks-tab-1">
<label for="pokemon-tabs-falinks-tab-1">Mega Falinks</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-falinks-panel-0">
Types: Fighting / Bug • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battle Armor
- Defiant *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0.5×)
- Bug (0.5×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Flying (4×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>

**Encounter Locations**
- Erinys Path (East) — Grass (Day) (10%)
- Erinys Path (East) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="falinks" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">470</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 1)
- <a href="move-lookup.html?q=rock-smash">Rock Smash</a> (Lv 5)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 10)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 14)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 17)
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a> (Lv 19)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 22)
- <a href="move-lookup.html?q=endure">Endure</a> (Lv 24)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 27)
- <a href="move-lookup.html?q=reversal">Reversal</a> (Lv 30)
- <a href="move-lookup.html?q=body-press">Body Press</a> (Lv 34)
- <a href="move-lookup.html?q=first-impression">First Impression</a> (Lv 36)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 38)
- <a href="move-lookup.html?q=no-retreat">No Retreat</a> (Lv 40)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 42)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 46)
- <a href="move-lookup.html?q=close-combat">Close Combat</a> (Lv 50)
- <a href="move-lookup.html?q=population-bomb">Population Bomb</a> (Lv 53)
- <a href="move-lookup.html?q=megahorn">Megahorn</a> (Lv 56)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-falinks-panel-1">
Types: Fighting / Steel • Egg Groups: Fairy / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Technician

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Bug (0.25×)
- Rock (0.25×)
- Dragon (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>

**Evolution Info**
Falinksite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-falinks" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">135</span> |
| Defense | <span class="stat-value stat-high">135</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-high">570</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 1)
- <a href="move-lookup.html?q=rock-smash">Rock Smash</a> (Lv 5)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 10)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 14)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 17)
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a> (Lv 19)
- <a href="move-lookup.html?q=bulk-up">Bulk Up</a> (Lv 22)
- <a href="move-lookup.html?q=endure">Endure</a> (Lv 24)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 27)
- <a href="move-lookup.html?q=reversal">Reversal</a> (Lv 30)
- <a href="move-lookup.html?q=body-press">Body Press</a> (Lv 34)
- <a href="move-lookup.html?q=first-impression">First Impression</a> (Lv 36)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 38)
- <a href="move-lookup.html?q=no-retreat">No Retreat</a> (Lv 40)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 42)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 46)
- <a href="move-lookup.html?q=close-combat">Close Combat</a> (Lv 50)
- <a href="move-lookup.html?q=population-bomb">Population Bomb</a> (Lv 53)
- <a href="move-lookup.html?q=megahorn">Megahorn</a> (Lv 56)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
#pokemon-tabs-falinks-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-falinks-panel-0 { display: block; }
#pokemon-tabs-falinks-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-falinks-panel-1 { display: block; }
</style>
</details>
