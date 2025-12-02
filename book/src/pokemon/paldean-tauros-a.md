<details class="pokemon-card-container">
<summary>Paldean Tauros-A (#224)</summary>
Types: Fighting / Water • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Anger Point
- Cud Chew *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm03-water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=tm07-whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=tm08-bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm26-earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="paldean-tauros-a" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=work-up">Work Up</a> (Lv 5)
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 10)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 15)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 20)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 25)
- <a href="move-lookup.html?q=zen-headbutt">Zen Headbutt</a> (Lv 30)
- <a href="move-lookup.html?q=raging-bull">Raging Bull</a> (Lv 35)
- <a href="move-lookup.html?q=rest">Rest</a> (Lv 40)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 45)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 50)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 55)
- <a href="move-lookup.html?q=close-combat">Close Combat</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=endeavor">Endeavor</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
