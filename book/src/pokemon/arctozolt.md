<details class="pokemon-card-container">
<summary>Arctozolt (#409)</summary>
Types: Electric / Ice • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Strong Jaw
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Ice (0.5×)
- Flying (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm13-ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=tm14-blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arctozolt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=powder-snow">Powder Snow</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 1)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 7)
- <a href="move-lookup.html?q=echoed-voice">Echoed Voice</a> (Lv 12)
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a> (Lv 16)
- <a href="move-lookup.html?q=aurora-beam">Aurora Beam</a> (Lv 20)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 24)
- <a href="move-lookup.html?q=avalanche">Avalanche</a> (Lv 28)
- <a href="move-lookup.html?q=ice-fang">Ice Fang</a> (Lv 33)
- <a href="move-lookup.html?q=thunder-fang">Thunder Fang</a> (Lv 35)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 38)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 40)
- <a href="move-lookup.html?q=freeze-dry">Freeze-Dry</a> (Lv 44)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 48)
- <a href="move-lookup.html?q=bolt-beak">Bolt Beak</a> (Lv 50)
- <a href="move-lookup.html?q=icicle-crash">Icicle Crash</a> (Lv 53)
- <a href="move-lookup.html?q=blizzard">Blizzard</a> (Lv 57)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
