<details class="pokemon-card-container">
<summary>Klawf (#117)</summary>
Types: Rock • Egg Groups: Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Anger Shell
- Shell Armor
- Regenerator *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Fighting (2×)
- Ground (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (2%)
- Acrisia Mountains — Grass (Night) (10%)
- Palati City — Grass (Day) (20%)
- Palati City — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="klawf" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-high">115</span> |
| Sp. Atk | <span class="stat-value stat-low">35</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">450</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=vise-grip">Vise Grip</a> (Lv 1)
- <a href="move-lookup.html?q=rock-throw">Rock Throw</a> (Lv 1)
- <a href="move-lookup.html?q=harden">Harden</a> (Lv 6)
- <a href="move-lookup.html?q=rock-smash">Rock Smash</a> (Lv 9)
- <a href="move-lookup.html?q=rock-tomb">Rock Tomb</a> (Lv 13)
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv 15)
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a> (Lv 17)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 21)
- <a href="move-lookup.html?q=rock-blast">Rock Blast</a> (Lv 24)
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a> (Lv 27)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 29)
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a> (Lv 33)
- <a href="move-lookup.html?q=crabhammer">Crabhammer</a> (Lv 35)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 37)
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a> (Lv 42)
- <a href="move-lookup.html?q=high-horsepower">High Horsepower</a> (Lv 45)
- <a href="move-lookup.html?q=strength">Strength</a> (Lv 48)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 51)
- <a href="move-lookup.html?q=guillotine">Guillotine</a> (Lv 56)

**Egg Moves**
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a>
- <a href="move-lookup.html?q=crabhammer">Crabhammer</a>
- <a href="move-lookup.html?q=endeavor">Endeavor</a>
- <a href="move-lookup.html?q=knock-off">Knock Off</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
</details>
