<details class="pokemon-card-container">
<summary>Aegislash Blade (#151)</summary>
Types: Steel / Ghost • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Stance Change

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.25×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Ground (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=cut">HM01 - Cut</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>

**Evolution Info**
Dusk Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="aegislash-blade" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-high">140</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-high">140</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=kings-shield">King's Shield</a> (Lv Evo)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a> (Lv 1)
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv 5)
- <a href="move-lookup.html?q=metal-sound">Metal Sound</a> (Lv 8)
- <a href="move-lookup.html?q=pursuit">Pursuit</a> (Lv 13)
- <a href="move-lookup.html?q=autotomize">Autotomize</a> (Lv 18)
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a> (Lv 20)
- <a href="move-lookup.html?q=aerial-ace">Aerial Ace</a> (Lv 22)
- <a href="move-lookup.html?q=retaliate">Retaliate</a> (Lv 26)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 29)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 32)
- <a href="move-lookup.html?q=night-slash">Night Slash</a> (Lv 35)
- <a href="move-lookup.html?q=power-trick">Power Trick</a> (Lv 39)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 42)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 45)
- <a href="move-lookup.html?q=sacred-sword">Sacred Sword</a> (Lv 47)
- <a href="move-lookup.html?q=bitter-blade">Bitter Blade</a> (Lv 50)
- <a href="move-lookup.html?q=might-cleave">Might Cleave</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=metal-sound">Metal Sound</a>
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a>
- <a href="move-lookup.html?q=destiny-bond">Destiny Bond</a>
- <a href="move-lookup.html?q=wide-guard">Wide Guard</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
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
</details>
